/* Str2table core crate for settings
 * Copyright (C) 2024 Peng Zijun, Xia Tingxuan
 *
 * This file is part of Str2table.
 * Foobar is free software: you can redistribute it and/or modify it under the terms of
 * the GNU General Public License as published by the Free Software Foundation, either
 * version 3 of the License, or (at your option) any later version.
 * Str2table is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
 * without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
 * See the GNU General Public License for more details.
 * You should have received a copy of the GNU General Public License along with Foobar.
 * If not, see <https://www.gnu.org/licenses/>.
 */

use super::core::LineColumn;
use crate::error::arg_error::{ArgError, ArgErrorKind};
use crate::error::conflicts::Conflicts;
use crate::error::keyword_missing::KeywordMissing;
use crate::error::range_error::{RangeError, RangeErrorKind};
use crate::error::{ErrorLevel, ErrorType};
use clap::Parser;
use clap::*;
use once_cell::sync::Lazy;
use regex::{Regex, RegexSet};
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Eq, Debug, ValueEnum)]
/// A enum to specify the parse mode, `A` represents auto, `S` represents force to string.
pub enum ParseMode {
    A,
    S,
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// A enum to specify the force type, `S` represents string, `I` represents integer,
/// `F` represents float.
pub enum ForceType {
    S,
    I,
    F,
}

impl FromStr for ForceType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "s" => Ok(ForceType::S),
            "i" => Ok(ForceType::I),
            "f" => Ok(ForceType::F),
            "S" => Ok(ForceType::S),
            "I" => Ok(ForceType::I),
            "F" => Ok(ForceType::F),
            _ => Err(()),
        }
    }
}

/// Commandline args
#[derive(Debug, PartialEq, Parser)]
pub struct InputArgs {
    #[arg(short, long, value_hint = clap::ValueHint::FilePath)]
    /// The path of input file, use console input if not set
    pub input: Option<std::path::PathBuf>,

    #[arg(short, long, default_value = " ")]
    /// Set the seperation pattern of the table, default is ` `, can be multiple chars
    pub seperation: String,

    #[arg(short, long, default_value = "\n")]
    /// Set the pattern to end the line, default is `\n`. if this is not `\n`,
    /// then all the `\n` and `\r` in the input will be removed first.
    pub end_line: String,

    #[arg(short, long, default_value = "a", value_enum)]
    pub parse_mode: ParseMode,

    #[arg(short, long, value_parser = validate_force_parse)]
    /// Give the lines or columns with specific type.
    pub force_parse: Option<(Vec<(usize, ForceType)>, super::LineColumn)>,
}

impl Default for InputArgs {
    fn default() -> Self {
        InputArgs {
            input: None,
            seperation: " ".to_string(),
            end_line: "\n".to_string(),
            parse_mode: ParseMode::A,
            force_parse: None,
        }
    }
}

fn validate_force_parse(s: &str) -> Result<(Vec<(usize, ForceType)>, super::LineColumn), ArgError> {
    // regex to check if input is valid
    let regex_set = RegexSet::new(&[
        // 0. correct format with a ragne
        r"^[0-9]+-[0-9]+[lcLC][sifSIF]$",
        // 1. correct format with a single number
        r"^[0-9]+[lcLC][sifSIF]$",
        // 2. wrong format with a wrong right side
        r"^[0-9]+-.*[lcLC][sifSIF]$",
        // 3. wrong format with a wrong left side
        r"^.*-[0-9]+[lcLC][sifSIF]$",
        // 4. wrong format with both side wrong
        r"^.*-.*[lcLC][sifSIF]$",
        // 5. wrong format with wrong number (single)
        r"^.*[lcLC][sifSIF]$",
        // 6. wrong format with wrong type (range)
        r"^[0-9]+-[0-9]+[lcLC].*$",
        // 7. wrong format with wrong type (single)
        r"^[0-9]+[lcLC].*$",
        // 8. wrong format with wrong line/column (range)
        r"^[0-9]+-[0-9]+.*[sifSIF]$",
        // 9. wrong format with wrong line/column (single)
        r"^[0-9]+.*[sifSIF]$",
    ])
    .unwrap();

    let mut result: Vec<(usize, ForceType)> = Vec::new();
    let mut linecolumn: Option<LineColumn> = None;

    // split the target string into parts
    let parts = s.split(',').map(|s| s.trim());

    // track the location of the error part
    let mut location = 0;

    // iterate through the parts
    for part in parts {
        let matches = regex_set.matches(part).into_iter().collect::<Vec<_>>();
        if matches.is_empty() {
            return Err(ArgError::new(
                ArgErrorKind::WrongFormat,
                Some("There is more than one error in this part.".to_string()),
                Some(part.to_string()),
                Some(s.to_string()),
                Some((location, location + part.len())),
                None,
            ));
        } else if matches[0] == 0 {
            match parse_range_force(part, s, (location, location + part.len()), linecolumn) {
                Ok((range, lc, force_type)) => {
                    linecolumn = Some(lc);
                    for i in range.0..=range.1 {
                        result.push((i, force_type));
                    }
                }
                Err(e) => return Err(e),
            }
        } else if matches[0] == 1 {
            match parse_single_force(part, s, (location, location + part.len()), linecolumn) {
                Ok((start, lc, force_type)) => {
                    linecolumn = Some(lc);
                    result.push((start, force_type));
                }
                Err(e) => return Err(e),
            }
        } else if matches[0] == 2 {
            let range_error = RangeError::new(
                RangeErrorKind::RightSideError,
                Some(part.to_string()),
                Some(s.to_string()),
                Some((location, location + part.len())),
            );
            return Err(ArgError::new(
                ArgErrorKind::WrongFormat,
                Some(range_error.message(ErrorLevel::Warning).to_string()),
                Some(part.to_string()),
                Some(s.to_string()),
                Some((location, location + part.len())),
                None,
            ));
        } else if matches[0] == 3 {
            let range_error = RangeError::new(
                RangeErrorKind::LeftSideError,
                Some(part.to_string()),
                Some(s.to_string()),
                Some((location, location + part.len())),
            );
            return Err(ArgError::new(
                ArgErrorKind::WrongFormat,
                Some(range_error.message(ErrorLevel::Warning).to_string()),
                Some(part.to_string()),
                Some(s.to_string()),
                Some((location, location + part.len())),
                None,
            ));
        } else if matches[0] == 4 {
            let range_error = RangeError::new(
                RangeErrorKind::BothSidesError,
                Some(part.to_string()),
                Some(s.to_string()),
                Some((location, location + part.len())),
            );
            return Err(ArgError::new(
                ArgErrorKind::WrongFormat,
                Some(range_error.message(ErrorLevel::Warning).to_string()),
                Some(part.to_string()),
                Some(s.to_string()),
                Some((location, location + part.len())),
                None,
            ));
        } else if matches[0] == 5 {
            let range_error = RangeError::new(
                RangeErrorKind::SingleNumberError,
                Some(part.to_string()),
                Some(s.to_string()),
                Some((location, location + part.len())),
            );
            return Err(ArgError::new(
                ArgErrorKind::WrongFormat,
                Some(range_error.message(ErrorLevel::Warning).to_string()),
                Some(part.to_string()),
                Some(s.to_string()),
                Some((location, location + part.len())),
                None,
            ));
        } else if matches[0] == 6 || matches[0] == 7 {
            let keyword_missing = KeywordMissing::new(
                Some(part.to_string()),
                Some(s.to_string()),
                Some((location, location + part.len())),
                "type".to_string(),
            );
            return Err(ArgError::new(
                ArgErrorKind::WrongFormat,
                Some(keyword_missing.message(ErrorLevel::Warning).to_string()),
                Some(part.to_string()),
                Some(s.to_string()),
                Some((location, location + part.len())),
                None,
            ));
        } else if matches[0] == 8 || matches[0] == 9 {
            let keyword_missing = KeywordMissing::new(
                Some(part.to_string()),
                Some(s.to_string()),
                Some((location, location + part.len())),
                "line or column".to_string(),
            );
            return Err(ArgError::new(
                ArgErrorKind::WrongFormat,
                Some(keyword_missing.message(ErrorLevel::Warning).to_string()),
                Some(part.to_string()),
                Some(s.to_string()),
                Some((location, location + part.len())),
                None,
            ));
        }
        location += part.len() + 1;
    }
    if let Some(linecolumn) = linecolumn {
        //sort and unique
        result.sort_by(|a, b| a.0.cmp(&b.0));
        result.dedup();
        assert_ne!(result.len(), 0);
        for i in 0..(result.len() - 1) {
            if result[i].0 == result[i + 1].0 {
                let conflict = Conflicts::new(
                    ErrorLevel::Error,
                    Some(s.to_string()),
                    Some(s.to_string()),
                    None,
                    Some(vec![
                        format!("{:?}", result[i]),
                        format!("{:?}", result[i + 1]),
                    ]),
                );
                return Err(ArgError::new(
                    ArgErrorKind::Conflicts,
                    Some(conflict.message(ErrorLevel::Warning)),
                    Some(s.to_string()),
                    Some(s.to_string()),
                    None,
                    Some("Please check the reason.".to_string()),
                ));
            }
        }
        Ok((result, linecolumn))
    } else {
        let keyword_missing = KeywordMissing::new(
            Some(s.to_string()),
            Some(s.to_string()),
            None,
            "line or column".to_string(),
        );
        return Err(ArgError::new(
            ArgErrorKind::WrongFormat,
            Some(keyword_missing.message(ErrorLevel::Warning).to_string()),
            Some(s.to_string()),
            Some(s.to_string()),
            None,
            None,
        ));
    }
}

fn parse_single_force(
    part: &str,
    whole_arg: &str,
    location: (usize, usize),
    linecolumn: Option<LineColumn>,
) -> Result<(usize, LineColumn, ForceType), ArgError> {
    static RE: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"^(?<start>[0-9]+)(?<lc>[lcLC])(?<type>[sifSIF])$").unwrap());
    let caps = RE.captures(part).unwrap();
    let start = caps["start"].parse::<usize>().unwrap();
    let lc = LineColumn::from_str(&caps["lc"]).unwrap();
    let force_type = ForceType::from_str(&caps["type"]).unwrap();
    if Some(lc) != linecolumn && linecolumn.is_some() {
        let conflict = Conflicts::new(
            ErrorLevel::Error,
            Some(part.to_string()),
            Some(whole_arg.to_string()),
            Some(location),
            Some(vec!['l'.to_string(), 'c'.to_string()]),
        );
        Err(ArgError::new(
            ArgErrorKind::WrongFormat,
            Some(conflict.message(ErrorLevel::Warning)),
            Some(part.to_string()),
            Some(whole_arg.to_string()),
            Some(location),
            None,
        ))
    } else {
        Ok((start, lc, force_type))
    }
}

fn parse_range_force(
    part: &str,
    whole_arg: &str,
    location: (usize, usize),
    linecolumn: Option<LineColumn>,
) -> Result<((usize, usize), LineColumn, ForceType), ArgError> {
    static RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"^(?<start>[0-9]+)-(?<end>[0-9]+)(?<lc>[lcLC])(?<type>[sifSIF])$").unwrap()
    });
    let caps = RE.captures(part).unwrap();
    let start = caps["start"].parse::<usize>().unwrap();
    let end = caps["end"].parse::<usize>().unwrap();
    let lc = LineColumn::from_str(&caps["lc"]).unwrap();
    let force_type = ForceType::from_str(&caps["type"]).unwrap();
    if Some(lc) != linecolumn && linecolumn.is_some() {
        let conflict = Conflicts::new(
            ErrorLevel::Error,
            Some(part.to_string()),
            Some(whole_arg.to_string()),
            Some(location),
            Some(vec!['l'.to_string(), 'c'.to_string()]),
        );
        Err(ArgError::new(
            ArgErrorKind::WrongFormat,
            Some(conflict.message(ErrorLevel::Warning)),
            Some(part.to_string()),
            Some(whole_arg.to_string()),
            Some(location),
            None,
        ))
    } else {
        Ok(((start, end), lc, force_type))
    }
}
