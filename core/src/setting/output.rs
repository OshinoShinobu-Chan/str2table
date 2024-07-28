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

use std::str::FromStr;

use crate::error::arg_error::{ArgError, ArgErrorKind};
use crate::error::keyword_missing::KeywordMissing;
use crate::error::range_error::{RangeError, RangeErrorKind};
use crate::error::{ErrorLevel, ErrorType};
use crate::setting::LineColumn;
use clap::Parser;
use clap::*;
use once_cell::sync::Lazy;
use regex::{Regex, RegexSet};

#[derive(Debug, Clone, Copy, PartialEq)]
/// A enum to specify the output format,
/// `Csv` represents csv file, `Txt` represents txt file, `Exls` represents excel file.
pub enum OutputFormat {
    Csv,
    Txt,
    Exls,
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// A enum to specify the color of the table.
pub enum OutputColor {
    Black,
    Red,
    Green,
    Blue,
    Yellow,
    Grey,
    White,
}

impl FromStr for OutputColor {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "r" | "R" => Ok(OutputColor::Red),
            "g" | "G" => Ok(OutputColor::Green),
            "b" | "B" => Ok(OutputColor::Blue),
            "y" | "Y" => Ok(OutputColor::Yellow),
            "x" | "X" => Ok(OutputColor::Grey),
            "w" | "W" => Ok(OutputColor::White),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for OutputColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputColor::Black => write!(f, "Black"),
            OutputColor::Red => write!(f, "Red"),
            OutputColor::Green => write!(f, "Green"),
            OutputColor::Blue => write!(f, "Blue"),
            OutputColor::Yellow => write!(f, "Yellow"),
            OutputColor::Grey => write!(f, "Grey"),
            OutputColor::White => write!(f, "White"),
        }
    }
}

impl Default for OutputColor {
    fn default() -> Self {
        OutputColor::Black
    }
}

#[derive(Debug, PartialEq, Parser)]
pub struct OutputArgs {
    #[command(flatten)]
    pub output_settings: OutputSettings,
}

impl Default for OutputArgs {
    fn default() -> Self {
        OutputArgs {
            output_settings: OutputSettings::default(),
        }
    }
}

#[derive(Args, Debug, PartialEq)]
#[group(multiple = false)]
pub struct OutputSettings {
    #[arg(short, long, value_parser = validate_output, value_hint = clap::ValueHint::FilePath)]
    /// The path of output file, use console output if not set, infer the format
    /// by the suffix of the file
    pub output: Option<(String, OutputFormat)>,

    #[arg(short = 'C', long, value_parser = validate_export_color)]
    /// Set the color of the table by line, enable when export mode is console
    /// ((line, color), (column, color))
    pub export_color: Option<(Vec<(usize, OutputColor)>, Vec<(usize, OutputColor)>)>,

    #[arg(short = 'S', long, value_parser = validate_export_subtable)]
    /// Use a number or range end with `l/c` to specify the line or column
    /// Export the subtable of the cross parts of the lines and columns
    pub export_subtable: Option<(Vec<usize>, Vec<usize>)>,
}

impl Default for OutputSettings {
    fn default() -> Self {
        OutputSettings {
            output: None,
            export_color: None,
            export_subtable: None,
        }
    }
}

fn validate_output(s: &str) -> Result<(String, OutputFormat), ArgError> {
    // Get the file format from suffix
    let parts: Vec<&str> = s.split('.').collect();
    let format = match parts[parts.len() - 1] {
        "csv" => OutputFormat::Csv,
        "txt" => OutputFormat::Txt,
        "xls" | "xlsx" => OutputFormat::Exls,
        _ => {
            return Err(ArgError::new(
                ArgErrorKind::FormatError,
                Some("The format of the file is not supported.".to_string()),
                Some(parts[parts.len() - 1].to_string()),
                Some(s.to_string()),
                None,
                None,
            ))
        }
    };

    Ok((s.to_string(), format))
}

fn validate_export_subtable(s: &str) -> Result<(Vec<usize>, Vec<usize>), ArgError> {
    // regex to check if input is valid
    let regex_set = RegexSet::new(&[
        // 0. correct range
        r"^[0-9]+-[0-9]+[lcLC]$",
        // 1. correct single
        r"^[0-9]+[lcLC]$",
        // 2. wrong format in left side of range
        r"^.*-[0-9]+[lcLC]$",
        // 3. wrong format in right side of range
        r"^[0-9]+-.*[lcLC]$",
        // 4. wrong format in both sides of range
        r"^.*-.*[lcLC]$",
        // 5. wrong format in single
        r"^.*[lcLC]$",
        // 6. wrong format in line/column (range)
        r"^[0-9]+-[0-9]+.*$",
        // 7. wrong format in line/column (single)
        r"^[0-9]+.*$",
    ])
    .unwrap();

    let mut lines: Vec<usize> = Vec::new();
    let mut columns: Vec<usize> = Vec::new();

    // split the target string into parts
    let parts = s.split(',');

    // track the location of the part
    let mut location = 0;

    // iterate through the parts
    for part in parts {
        let matches = regex_set.matches(part).into_iter().collect::<Vec<_>>();
        if matches.is_empty() {
            return Err(ArgError::new(
                ArgErrorKind::WrongFormat,
                Some("There is more than one error in this part".to_string()),
                Some(part.to_string()),
                Some(s.to_string()),
                Some((location, location + part.len())),
                None,
            ));
        } else if matches[0] == 0 {
            let ((start, end), lc) = parse_range_subtable(part);
            match lc {
                LineColumn::Line => {
                    for i in start..=end {
                        lines.push(i);
                    }
                }
                LineColumn::Column => {
                    for i in start..=end {
                        columns.push(i);
                    }
                }
            }
        } else if matches[0] == 1 {
            let (num, lc) = parse_single_subtable(part);
            match lc {
                LineColumn::Line => {
                    lines.push(num);
                }
                LineColumn::Column => {
                    columns.push(num);
                }
            }
        } else if matches[0] == 2 {
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
        } else if matches[0] == 3 {
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
                Some("line/column".to_string()),
                Some("export_subtable".to_string()),
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
        location += part.len();
    }
    Ok((lines, columns))
}

fn validate_export_color(
    s: &str,
) -> Result<(Vec<(usize, OutputColor)>, Vec<(usize, OutputColor)>), ArgError> {
    let regex_set = RegexSet::new(&[
        // 0. correct range
        r"^[0-9]+-[0-9]+[rgbyxwRGYBXW][lcLC]$",
        // 1. correct single
        r"^[0-9]+[rgbyxwRGYBXW][lcLC]$",
        // 2. wrong format in left side of range
        r"^.*-[0-9]+[rgbyxwRGYBXW][lcLC]$",
        // 3. wrong format in right side of range
        r"^[0-9]+-.*[rgbyxwRGYBXW][lcLC]$",
        // 4. wrong format in both sides of range
        r"^.*-.*[rgbyxwRGYBXW][lcLC]$",
        // 5. wrong format in single
        r"^.*[rgbyxwRGYBXW][lcLC]$",
        // 6. wrong format in line/column (range)
        r"^[0-9]+-[0-9]+[rgbyxwRGBYXW].*$",
        // 7. wrong format in line/column (single)
        r"^[0-9]+[rgbyxwRGBYXW].*$",
        // 8. wrong format in color
        r"^[0-9]+-[0-9]+.*[lcLC]$",
    ])
    .unwrap();
    let mut lines: Vec<(usize, OutputColor)> = Vec::new();
    let mut columns: Vec<(usize, OutputColor)> = Vec::new();

    let parts = s.split(',');
    let mut location = 0;

    for part in parts {
        let matches = regex_set.matches(part).into_iter().collect::<Vec<_>>();
        if matches.is_empty() {
            return Err(ArgError::new(
                ArgErrorKind::WrongFormat,
                Some("There is more than one error in this part".to_string()),
                Some(part.to_string()),
                Some(s.to_string()),
                Some((location, location + part.len())),
                None,
            ));
        } else if matches[0] == 0 {
            let ((start, end), color, lc) = parse_range_color(part);
            match lc {
                LineColumn::Line => {
                    for i in start..=end {
                        lines.push((i, color));
                    }
                }
                LineColumn::Column => {
                    for i in start..=end {
                        columns.push((i, color));
                    }
                }
            }
        } else if matches[0] == 1 {
            let (num, color, lc) = parse_single_color(part);
            match lc {
                LineColumn::Line => {
                    lines.push((num, color));
                }
                LineColumn::Column => {
                    columns.push((num, color));
                }
            }
        } else if matches[0] == 2 {
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
        } else if matches[0] == 3 {
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
                Some("line/column".to_string()),
                Some("export_color".to_string()),
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
        } else if matches[0] == 8 {
            let keyword_missing = KeywordMissing::new(
                Some("color".to_string()),
                Some("export_color".to_string()),
                Some((location, location + part.len())),
                "color".to_string(),
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
        location += part.len();
    }
    Ok((lines, columns))
}

fn parse_range_subtable(s: &str) -> ((usize, usize), LineColumn) {
    static RE: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"(?<start>[0-9]+)-(?<end>[0-9]+)(?<lc>[lcLC])").unwrap());
    let caps = RE.captures(s).unwrap();
    let start = caps["start"].parse::<usize>().unwrap();
    let end = caps["end"].parse::<usize>().unwrap();
    let lc = LineColumn::from_str(&caps["lc"]).unwrap();
    ((start, end), lc)
}

fn parse_single_subtable(s: &str) -> (usize, LineColumn) {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?<num>[0-9]+)(?<lc>[lcLC])").unwrap());
    let caps = RE.captures(s).unwrap();
    let num = caps["num"].parse::<usize>().unwrap();
    let lc = LineColumn::from_str(&caps["lc"]).unwrap();
    (num, lc)
}

fn parse_range_color(s: &str) -> ((usize, usize), OutputColor, LineColumn) {
    static RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"(?<start>[0-9]+)-(?<end>[0-9]+)(?<color>[rgbyxwRGYBXW])(?<lc>[lcLC])").unwrap()
    });
    let caps = RE.captures(s).unwrap();
    let start = caps["start"].parse::<usize>().unwrap();
    let end = caps["end"].parse::<usize>().unwrap();
    let color = OutputColor::from_str(&caps["color"]).unwrap();
    let lc = LineColumn::from_str(&caps["lc"]).unwrap();
    ((start, end), color, lc)
}

fn parse_single_color(s: &str) -> (usize, OutputColor, LineColumn) {
    static RE: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"(?<num>[0-9]+)(?<color>[rgbyxwRGYBXW])(?<lc>[lcLC])").unwrap());
    let caps = RE.captures(s).unwrap();
    let num = caps["num"].parse::<usize>().unwrap();
    let color = OutputColor::from_str(&caps["color"]).unwrap();
    let lc = LineColumn::from_str(&caps["lc"]).unwrap();
    (num, color, lc)
}
