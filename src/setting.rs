//! # Setting
//! This module is used to set the setting of the table. Settings can be set by
//! commandline arguments temperarily or by a pre-set configuration file. You can
//! also mix the two ways to set the setting.
//!
//! ## Configuration Options
//! - `Input Path`: The path of input file, use console input if not set
//! - `seperation`: The seperation char of the table
//! - `parse mode`: Whether to parse the cell to auto type, or force to string
//! - `force parse`: Force a line or a column or a cell to be parsed to a specific type
//! - `export path`: The path of file to export the table, enable when export mode is not console
//! - `export color`: Set the color of the table when export, by line or by column, enable when export mode is console
//! - `export subtable`: Export a subtable of the table
//!
//! ## Commandline Options
//! - `-i`: Set the input path of the table, use console input if not set
//! - `-s`/`--seperation`: Set the seperation char of the table, default is ` `, can be multiple chars
//! - `-pm`/`--parse-mode`: Set the parse mode of the table, default is `a`(auto), can be `a` or `s`
//! - `-fp`/`--force-parse`: Give the lines or columns with specific type.
//! Use number or range end with `l/c` to specify the line or column.
//! And only one number or range include `l/c` is ok.
//! Use `x-y` to specify the range, `x` and `y` are both inclusive.
//! Use `s/u/i/f` to specify the type, `s` for string, `i` for integer, `f` for float, at the end of every part.
//! Use `,` to seperate the lines or columns, and do not use space
//! Panic if the the force type is conflict.
//! Panic if `l` and `c` are both used in this arguement.
//! If the force type has error, then use auto_parse.
//! Lines or columns that do not exist will be ignored.
//! - `-o`/`--output`: Set the path of file to export the table, enable when export mode is not console.
//! Infer the format by the suffix of the file, support `csv`, `txt`, `exls`.
//! - `-ec`/`--export-color`: Set the color of the table by line, enable when export mode is console
//! Use number or range end with `l/x` and with color, default is black.
//! `r` represents red, `g` represents green, `b` represents blue, `y` represents yellow, `x` represents grey
//! `w` represents white.
//! Follow the line color first if conflict.
//! - `-es`/`--export-subtable`: Set the subtable to export, default is the whole table.
//! Use number or range end with `l/c` to specify the line or column.
//! Export the subtable of the cross parts of the lines and columns.
//! - `-c`/`--config`: Set the configuration file to use and the configuration name
//! you want to use. Use the configuration from the commandline first if conflict.
//! - `-d`/`--dry`: Export the setting to the given toml file, but not run the program.
//! - `-h`/`--help`: Print the help message.
//!
//! ### Example
//! ```bash
//! str2table -s '#' -pm s -fp 1-2li,4f -ecl 1r,2g,3b -es 1-3l,1-3c
//! ```
//! This command means, read a table from console with `#` as seperation char,
//! parse the table to string, force the first two lines to be integer, and fourth lines to be float
//! export the table to concole`, set the color
//! of the first line to red, the second line to green, the third line to blue,
//! export the subtable of the first three lines and the first three columns.
//!
//! ## Configuration File
//! The configuration file is a toml file, with the following format:
//! ```toml
//! # Configuration file for str2table
//! # You can use conf_name to set the name of the configuration
//! # So you can include multiple configuration in one file
//! [conf_name]
//! # input path, use console input if not set
//! input = "input.txt"
//!
//! # seperation char, default is ' '
//! seperation = "#"
//!
//! # Is auto parse, default is true
//! is_auto = true
//!
//! # force parse line, use a array, default is []
//! # the following example means, force the first line to string,
//! # the second line to fourth line to integer
//! force_parse.line = [
//! [1, 1, 's'],
//! [2, 4, 'i'],
//! ]
//!
//! # force parse column, use a array, default is [], same as line
//! force_parse.column = [
//! [1, 1, 's'],
//! [2, 2, 'i'],
//! ]
//!
//! # export path, use console output if not set
//! export = "output.txt"
//!
//! # export color by line, use a array, default is []
//! # the following example means, set the first line to red,
//! # the second line to fourth line to green, the third line to blue
//! export_color.line = [
//! [1, 1, 'r'],
//! [2, 4, 'g'],
//! ]
//!
//! # export color by column, use a array, default is [], same as line
//! export_color.column = [
//! [1, 1, 'r'],
//! [2, 2, 'g'],
//! ]
//!
//! # export subtable line, use a array, default export the whole line
//! # the following example means, export the first line and third line
//! export_subtable.line = [1, 3]
//!
//! # export subtable column, use a array, default export the whole column
//! # the following example means, export the first column and third column
//! export_subtable.column = [1, 3]
//!
//! # not export subtable line, use a array, default is [] means export the whole column
//! # the following example means, not export the first and third line
//! # can't set it with export_subtable.line
//! # not_export_subtable.line = [1, 3]
//!
//! # not export subtable column, use a array, default is [] means export the whole line
//! # the following example means, not export the first and third column
//! # can't set it with export_subtable.column
//! # not_export_subtable.column = [1, 3]
//!
//! # use configuration from other configuration module, use config from this configuration first if conflict
//! # if you use . as path, then find the conf_name in this file
//! configuration = ["path/to/file", "conf_name"]
//! ```

use std::fmt::format;

use clap::Parser;
use clap::*;
#[derive(Clone, Copy, PartialEq, Eq, Debug, ValueEnum)]
enum ParseMode {
    A,
    S,
}

#[derive(Debug, Clone, Copy)]
enum ForceType {
    S,
    U,
    I,
    F,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LineColumn {
    Line,
    Column,
}

#[derive(Debug, Clone, Copy)]
enum OutputFormat {
    Csv,
    Txt,
    Exls,
}

#[derive(Debug, Clone, Copy)]
enum OutputColor {
    Red,
    Green,
    Blue,
    Yellow,
    Grey,
    White,
}

/// Commandline args
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    /// The path of input file, use console input if not set
    input: Option<String>,

    #[arg(short, long, default_value = " ")]
    seperation: String,

    #[arg(short, long, default_value = "a", value_enum)]
    parse_mode: ParseMode,

    #[arg(short, long, value_parser = validate_force_parse)]
    /// Give the lines or columns with specific type.
    force_parse: Option<(Vec<(usize, ForceType)>, LineColumn)>,

    #[command(flatten)]
    output_settings: OutputSettings,

    #[arg(short = 'E', long, value_parser = validate_export_subtable)]
    /// Use a number or range end with `l/c` to specify the line or column
    /// Export the subtable of the cross parts of the lines and columns
    export_subtable: Option<(Vec<usize>, Vec<usize>)>,

    #[arg(short, long)]
    /// Set the configuration file to use and the configuration name you want to use
    /// Use the configuration from the commandline first if conflict
    config: Option<Vec<String>>,

    #[arg(short, long)]
    /// Export the setting to the given toml file, but not run the program
    dry: Option<String>,
}

#[derive(Args, Debug)]
#[group(multiple = false)]
struct OutputSettings {
    #[arg(short, long, value_parser = validate_output)]
    /// The path of output file, use console output if not set, infer the format
    /// by the suffix of the file
    output: Option<(String, OutputFormat)>,

    #[arg(short = 'C', long, value_parser = validate_export_color)]
    /// Set the color of the table by line, enable when export mode is console
    export_color: Option<(Vec<(usize, OutputColor)>, Vec<(usize, OutputColor)>)>,
}

fn validate_force_parse(s: &str) -> Result<(Vec<(usize, ForceType)>, LineColumn), String> {
    let parts = s.split(',');
    let mut lc: Option<LineColumn> = None;
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
            let last: char = range[1].chars().last().unwrap();
            let second_last: char = range[1].chars().nth(range[1].len() - 2).unwrap();
            // show if the lc is included in this part
            let mut lc_flag = true;

            match second_last {
                'l' => {
                    if let Some(lc) = lc {
                        if lc == LineColumn::Column {
                            return Err(format!(
                                "'\x1b[1;31m{}\x1b[0m' can't use 'l' and 'c' at the same time",
                                part
                            ));
                        }
                    } else {
                        lc = Some(LineColumn::Line);
                    }
                }
                'c' => {
                    if let Some(lc) = lc {
                        if lc == LineColumn::Line {
                            return Err(format!(
                                "'\x1b[1;31m{}\x1b[0m' can't use 'l' and 'c' at the same time",
                                part
                            ));
                        }
                    } else {
                        lc = Some(LineColumn::Column);
                    }
                }
                _ => lc_flag = false,
            }

            match last {
                's' => t = ForceType::S,
                'u' => t = ForceType::U,
                'i' => t = ForceType::I,
                'f' => t = ForceType::F,
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
            let last: char = part.chars().last().unwrap();
            let second_last: char = part.chars().nth(part.len() - 2).unwrap();
            let mut lc_flag = true;

            match second_last {
                'l' => {
                    if let Some(lc) = lc {
                        if lc == LineColumn::Column {
                            return Err(format!(
                                "'\x1b[1;31m{}\x1b[0m' can't use 'l' and 'c' at the same time",
                                part
                            ));
                        }
                    } else {
                        lc = Some(LineColumn::Line);
                    }
                }
                'c' => {
                    if let Some(lc) = lc {
                        if lc == LineColumn::Line {
                            return Err(format!(
                                "'\x1b[1;31m{}\x1b[0m' can't use 'l' and 'c' at the same time",
                                part
                            ));
                        }
                    } else {
                        lc = Some(LineColumn::Column);
                    }
                }
                _ => lc_flag = false,
            }

            match last {
                's' => t = ForceType::S,
                'u' => t = ForceType::U,
                'i' => t = ForceType::I,
                'f' => t = ForceType::F,
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

fn validate_output(s: &str) -> Result<(String, OutputFormat), String> {
    // Get the file format from suffix
    let parts: Vec<&str> = s.split('.').collect();
    let format = match parts[parts.len() - 1] {
        "csv" => OutputFormat::Csv,
        "txt" => OutputFormat::Txt,
        "exls" => OutputFormat::Exls,
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
    // TODO
    println!("{:?}", s);
    Ok((Vec::new(), Vec::new()))
}

fn validate_export_subtable(s: &str) -> Result<(Vec<usize>, Vec<usize>), String> {
    // TODO
    println!("{:?}", s);
    Ok((Vec::new(), Vec::new()))
}
