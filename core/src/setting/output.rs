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
    pub export_color: Option<(Vec<(usize, OutputColor)>, Vec<(usize, OutputColor)>)>,
}

impl Default for OutputSettings {
    fn default() -> Self {
        OutputSettings {
            output: None,
            export_color: None,
        }
    }
}

fn validate_output(s: &str) -> Result<(String, OutputFormat), String> {
    // Get the file format from suffix
    let parts: Vec<&str> = s.split('.').collect();
    let format = match parts[parts.len() - 1] {
        "csv" => OutputFormat::Csv,
        "txt" => OutputFormat::Txt,
        "xls" | "xlsx" => OutputFormat::Exls,
        _ => {
            return Err(format!(
                "Unsupported file format '\x1b[1;31m{}\x1b[0m'",
                parts[parts.len() - 1]
            ))
        }
    };

    Ok((s.to_string(), format))
}

fn validate_export_color(
    s: &str,
) -> Result<(Vec<(usize, OutputColor)>, Vec<(usize, OutputColor)>), String> {
    let parts = s.split(',');
    let mut line: Vec<(usize, OutputColor)> = Vec::new();
    let mut column: Vec<(usize, OutputColor)> = Vec::new();
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
            let color: OutputColor;

            if range[1].len() <= 2 {
                return Err(format!("'\x1b[1;31m{}\x1b[0m' invalid format", part));
            }

            let last = range[1].chars().last();
            let second_last = range[1].chars().nth(range[1].len() - 2);
            let is_line: bool;

            match second_last {
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

            match last {
                Some('r') => color = OutputColor::Red,
                Some('g') => color = OutputColor::Green,
                Some('b') => color = OutputColor::Blue,
                Some('y') => color = OutputColor::Yellow,
                Some('x') => color = OutputColor::Grey,
                Some('w') => color = OutputColor::White,
                _ => {
                    return Err(format!(
                    "'\x1b[1;31m{}\x1b[0m' should end with color 'r', 'g', 'b', 'y', 'x' or 'w'",
                    range[1]
                ))
                }
            }

            match range[1][..range[1].len() - 2].parse::<usize>() {
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

            // put the result to vec
            if is_line {
                for i in start..=end {
                    line.push((i, color));
                }
            } else {
                for i in start..=end {
                    column.push((i, color));
                }
            }
        } else {
            // part is a number
            let num: usize;
            let color: OutputColor;
            if part.len() <= 2 {
                return Err(format!("'\x1b[1;31m{}\x1b[0m' invalid format", part));
            }
            let last = part.chars().last();
            let second_last = part.chars().nth(part.len() - 2);
            let is_line: bool;

            match second_last {
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

            match last {
                Some('r') => color = OutputColor::Red,
                Some('g') => color = OutputColor::Green,
                Some('b') => color = OutputColor::Blue,
                Some('y') => color = OutputColor::Yellow,
                Some('x') => color = OutputColor::Grey,
                Some('w') => color = OutputColor::White,
                _ => {
                    return Err(format!(
                    "'\x1b[1;31m{}\x1b[0m' should end with color 'r', 'g', 'b', 'y', 'x' or 'w'",
                    part
                ))
                }
            }

            match part[..part.len() - 2].parse::<usize>() {
                Ok(n) => num = n,
                Err(e) => return Err(format!("'\x1b[1;31m{}\x1b[0m' has {}", part, e.to_string())),
            }

            // put the result to vec
            if is_line {
                line.push((num, color));
            } else {
                column.push((num, color));
            }
        }
    }
    // sort the lines and columns by number
    line.sort_by(|a, b| a.0.cmp(&b.0));
    column.sort_by(|a, b| a.0.cmp(&b.0));
    Ok((line, column))
}
