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

use clap::Parser;
use clap::*;

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

    #[arg(short = 'S', long, value_parser = validate_export_subtable)]
    /// Use a number or range end with `l/c` to specify the line or column
    /// Export the subtable of the cross parts of the lines and columns
    pub export_subtable: Option<(Vec<usize>, Vec<usize>)>,
}

impl Default for InputArgs {
    fn default() -> Self {
        InputArgs {
            input: None,
            seperation: " ".to_string(),
            end_line: "\n".to_string(),
            parse_mode: ParseMode::A,
            force_parse: None,
            export_subtable: None,
        }
    }
}

fn validate_force_parse(s: &str) -> Result<(Vec<(usize, ForceType)>, super::LineColumn), String> {
    let parts = s.split(',');
    let mut lc: Option<super::LineColumn> = None;
    let mut result: Vec<(usize, ForceType)> = Vec::new();
    for part in parts {
        // if part is a range
        if part.contains('-') {
            let range: Vec<&str> = part.split('-').collect();
            // parse start of range
            let start: usize;
            match range[0].parse::<usize>() {
                Ok(n) => start = n,
                Err(e) => {
                    return Err(format!(
                        "'\x1b[1;31m{}\x1b[0m' has {}",
                        range[0],
                        e.to_string()
                    ))
                }
            }

            // parse end of range
            let end: usize;
            let t: ForceType;
            let last = range[1].chars().last();
            if range[1].len() < 2 {
                return Err(format!("'\x1b[1;31m{}\x1b[0m' invalid format", part));
            }
            let second_last = range[1].chars().nth(range[1].len() - 2);
            // show if the lc is included in this part
            let mut lc_flag = true;

            match second_last {
                Some('l') => {
                    if let Some(lc) = lc {
                        if lc == super::LineColumn::Column {
                            return Err(format!(
                                "'\x1b[1;31m{}\x1b[0m' can't use 'l' and 'c' at the same time",
                                part
                            ));
                        }
                    } else {
                        lc = Some(super::LineColumn::Line);
                    }
                }
                Some('c') => {
                    if let Some(lc) = lc {
                        if lc == super::LineColumn::Line {
                            return Err(format!(
                                "'\x1b[1;31m{}\x1b[0m' can't use 'l' and 'c' at the same time",
                                part
                            ));
                        }
                    } else {
                        lc = Some(super::LineColumn::Column);
                    }
                }
                _ => lc_flag = false,
            }

            match last {
                Some('s') => t = ForceType::S,
                Some('i') => t = ForceType::I,
                Some('f') => t = ForceType::F,
                _ => {
                    return Err(format!(
                        "'\x1b[1;31m{}\x1b[0m' should end with type 's', 'u', 'i' or 'f'",
                        range[1]
                    ))
                }
            }

            let end_pos = if lc_flag && range[1].len() > 2 {
                range[1].len() - 2
            } else if range[1].len() > 1 {
                range[1].len() - 1
            } else {
                return Err(format!(
                    "'\x1b[1;31m{}\x1b[0m' lack of end number for range",
                    range[1]
                ));
            };
            match range[1][..end_pos].parse::<usize>() {
                Ok(n) => end = n,
                Err(e) => {
                    return Err(format!(
                        "'\x1b[1;31m{}\x1b[0m' has {}",
                        range[1],
                        e.to_string()
                    ))
                }
            }

            if start > end {
                return Err(format!(
                    "Start of range (\x1b[1;31m{}\x1b[0m) should be less than end (\x1b[1;31m{}\x1b[0m)",
                    start,
                    end,
                ));
            }
            for i in start..=end {
                result.push((i, t));
            }
        } else {
            // part is a number
            let num: usize;
            let t: ForceType;
            let last = part.chars().last();
            if part.len() < 2 {
                return Err(format!("'\x1b[1;31m{}\x1b[0m' invalid format", part));
            }
            let second_last = part.chars().nth(part.len() - 2);
            let mut lc_flag = true;

            match second_last {
                Some('l') => {
                    if let Some(lc) = lc {
                        if lc == super::LineColumn::Column {
                            return Err(format!(
                                "'\x1b[1;31m{}\x1b[0m' can't use 'l' and 'c' at the same time",
                                part
                            ));
                        }
                    } else {
                        lc = Some(super::LineColumn::Line);
                    }
                }
                Some('c') => {
                    if let Some(lc) = lc {
                        if lc == super::LineColumn::Line {
                            return Err(format!(
                                "'\x1b[1;31m{}\x1b[0m' can't use 'l' and 'c' at the same time",
                                part
                            ));
                        }
                    } else {
                        lc = Some(super::LineColumn::Column);
                    }
                }
                _ => lc_flag = false,
            }

            match last {
                Some('s') => t = ForceType::S,
                Some('i') => t = ForceType::I,
                Some('f') => t = ForceType::F,
                _ => {
                    return Err(format!(
                        "'\x1b[1;31m{}\x1b[0m' should end with type 's', 'u', 'i' or 'f'",
                        part
                    ))
                }
            }

            let end_pos = if lc_flag && part.len() > 2 {
                part.len() - 2
            } else if part.len() > 1 {
                part.len() - 1
            } else {
                return Err(format!(
                    "'\x1b[1;31m{}\x1b[0m' lack of number for range",
                    part
                ));
            };

            match part[..end_pos].parse::<usize>() {
                Ok(n) => num = n,
                Err(e) => return Err(format!("'\x1b[1;31m{}\x1b[0m' has {}", part, e.to_string())),
            }

            // put the result to vec
            result.push((num, t));
        }
    }
    // sort the lines and columns by number
    result.sort_by(|a, b| a.0.cmp(&b.0));

    // check conflicts
    for i in 0..result.len() - 1 {
        if result[i].0 == result[i + 1].0 {
            return Err(format!(
                "Conflict between '\x1b[1;31m{}\x1b[0m' and '\x1b[1;31m{}\x1b[0m'",
                result[i].0,
                result[i + 1].0
            ));
        }
    }

    if let Some(lc) = lc {
        Ok((result, lc))
    } else {
        Err("No line or column specified".to_string())
    }
}

fn validate_export_subtable(s: &str) -> Result<(Vec<usize>, Vec<usize>), String> {
    let parts = s.split(',');
    let mut line: Vec<usize> = Vec::new();
    let mut column: Vec<usize> = Vec::new();
    for part in parts {
        // if part is a range
        if part.contains('-') {
            let range = part.split('-').collect::<Vec<&str>>();
            // parse start of range
            let start: usize;
            match range[0].parse::<usize>() {
                Ok(n) => start = n,
                Err(e) => {
                    return Err(format!(
                        "'\x1b[1;31m{}\x1b[0m' has {}",
                        range[0],
                        e.to_string()
                    ))
                }
            }

            // parse end of range
            let end: usize;
            let is_line: bool;
            if range[1].len() <= 1 {
                return Err(format!("'\x1b[1;31m{}\x1b[0m' invalid format", part));
            }
            let last = range[1].chars().last();

            match last {
                Some('l') => is_line = true,
                Some('c') => is_line = false,
                Some(_) => {
                    return Err(format!(
                        "'\x1b[1;31m{}\x1b[0m' should end with 'l' or 'c'",
                        range[1]
                    ))
                }
                None => {
                    return Err(format!(
                        "'\x1b[1;31m{}\x1b[0m' lack of 'l' or 'c' to specify line or column",
                        range[1]
                    ))
                }
            }

            match range[1][..range[1].len() - 1].parse::<usize>() {
                Ok(n) => end = n,
                Err(e) => {
                    return Err(format!(
                        "'\x1b[1;31m{}\x1b[0m' has {}",
                        range[1],
                        e.to_string()
                    ))
                }
            }

            if start > end {
                return Err(format!(
                    "Start of range (\x1b[1;31m{}\x1b[0m) should be less than end (\x1b[1;31m{}\x1b[0m)",
                    start,
                    end,
                ));
            }
            for i in start..=end {
                if is_line {
                    line.push(i);
                } else {
                    column.push(i);
                }
            }
        } else {
            // part is a number
            let num: usize;
            let is_line: bool;
            if part.len() <= 1 {
                return Err(format!("'\x1b[1;31m{}\x1b[0m' invalid format", part));
            }
            let last = part.chars().last();
            match last {
                Some('l') => is_line = true,
                Some('c') => is_line = false,
                Some(_) => {
                    return Err(format!(
                        "'\x1b[1;31m{}\x1b[0m' should end with 'l' or 'c'",
                        part
                    ))
                }
                None => {
                    return Err(format!(
                        "'\x1b[1;31m{}\x1b[0m' lack of 'l' or 'c' to specify line or column",
                        part
                    ))
                }
            }

            match part[..part.len() - 1].parse::<usize>() {
                Ok(n) => num = n,
                Err(e) => return Err(format!("'\x1b[1;31m{}\x1b[0m' has {}", part, e.to_string())),
            }

            // put the result to vec
            if is_line {
                line.push(num);
            } else {
                column.push(num);
            }
        }
    }
    // sort the lines and columns by number
    line.sort();
    column.sort();

    Ok((line, column))
}
