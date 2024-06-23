//! # Setting
//! This module is used to set the setting of the table. Settings can be set by
//! commandline arguments temperarily or by a pre-set configuration file. You can
//! also mix the two ways to set the setting.
//!
//! ## Configuration Options
//! - `input`: The path of input file, use console input if not set
//! - `seperation`: The seperation char of the table
//! - `parse_mode`: Whether to parse the cell to auto type, or force to string
//! - `force_parse`: Force a line or a column or a cell to be parsed to a specific type
//! - `output`: The path of file to export the table, enable when export mode is not console
//! - `export_color`: Set the color of the table when export, by line or by column, enable when export mode is console
//! - `export_subtable`: Export a subtable of the table
//!
//! ## Commandline Options
//! - `-i` `<INPUT>`: Set the input path of the table as `<INPUT>`, use console input if not set
//! - `-s`/`--seperation` `<SEPERATION>`: Set the seperation pattern of the table as `<SEPERATION>`, default is ` `, can be multiple chars
//! - `-e`/`--end-line` `<END_LINE>`: Set the pattern to end the line as `<END_LINE>`, default is `\n`
//! - `-p`/`--parse-mode` `<PARSE_MODE>`: Set the parse mode of the table as `<PARSE_MODE>`, default is `a`(auto), can be `a` or `s`
//! - `-f`/`--force-parse` `<FORCE_PARSE>`: Force the lines or columns in `<FORCE_PARSE>` to be parsed as specific type.
//! Use number or range end with `l/c` to specify the line or column.
//! And only one number or range include `l/c` is ok.
//! Use `x-y` to specify the range, `x` and `y` are both included
//! Use `s/u/i/f` to specify the type, `s` for string, `i` for integer, `f` for float, at the end of every part.
//! Use `,` to seperate the lines or columns, and do not use space
//! Panic if the the force type is conflict.
//! Panic if `l` and `c` are both used in this arguement.
//! If the force type has error, then use auto_parse.
//! Lines or columns that do not exist will be ignored.
//! - `-o`/`--output` `<OUTPUT>`: Set the path of file to export the table as `<OUTPUT>`, enable when export mode is not console.
//! Infer the format by the suffix of the file, support `csv`, `txt`, `exls`.
//! - `-C`/`--export-color` `<EXPORT_COLOR>`: Set the color of the table by line, enable when export mode is console
//! Use number or range end with `l/c` and with color, default is black.
//! `r` represents red, `g` represents green, `b` represents blue, `y` represents yellow, `x` represents grey
//! `w` represents white.
//! Follow the line color first if conflict.
//! - `-S`/`--export-subtable` `<EXPORT_SUBTABLE>`: Set the subtable to export, default is the whole table.
//! Use number or range end with `l/c` to specify the line or column.
//! Export the subtable of the cross parts of the lines and columns.
//! - `-c`/`--config` `<EXPORT_PATH>`: Set the configuration file to use as `<EXPORT_PATH>`.
//! Use the configuration from the commandline first if conflict.
//! - `-n`/`--config-name` `<EXPORT_NAME>`: Set the configuration name you want to use in the configuration file as `<EXPORT_NAME>`.
//! - `-d`/`--dry` `<DRY>` : Export the setting to the given toml file `<DRY>` , but not run the program.
//! - `-h`/`--help`: Print the help message.
//!
//! ### Example
//! ```bash
//! str2table -s '#' -pm s -fp 1-2li,4f -ecl 1lr,2lg,3cb -es 1-3l,1-3c
//! ```
//! This command means, read a table from console with `#` as seperation char,
//! parse the table to string, force the first two lines to be integer, and fourth lines to be float
//! export the table to concole`, set the color
//! of the first line to red, the second line to green, the third column to blue,
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
//! # if set to false, force all the data to str
//! is_auto = true
//!
//! # force parse line, use an array, default is []
//! # the following example means, force the first line to string,
//! # the second line to fourth line to integer
//! force_parse.line = [
//! [1, 1, 's'],
//! [2, 4, 'i'],
//! ]
//!
//! # force parse column, use an array, default is [], same as line
//! # this can't be used with force_parse.line
//! # force_parse.column = [
//! # [1, 1, 's'],
//! # [2, 2, 'i'],
//! # ]
//!
//! # export path, use console output if not set
//! export_path = "output.txt"
//!
//! # export color by line, use an array, default is []
//! # the following example means, set the first line to red,
//! # the second line to fourth line to green, the third line to blue
//! export_color.line = [
//! [1, 1, 'r'],
//! [2, 4, 'g'],
//! ]
//!
//! # export color by column, use an array, default is [], same as line
//! export_color.column = [
//! [1, 1, 'r'],
//! [2, 2, 'g'],
//! ]
//!
//! # export subtable line, use an array, default export the whole line
//! # you can also use an array of two to represent a range
//! # the following example means, export the first line and third line
//! export_subtable.line = [[1, 1] , [3, 3] ]
//!
//! # export subtable column, use an array, default export the whole column
//! # you can also use an array of two to represent a range
//! # the following example means, export the first to second columns and fourth column
//! export_subtable.column = [[1, 2], [4, 4]]
//!
//! # use configuration from other configuration module, use config from this configuration first if conflict
//! # if you use . as path, then find the conf_name in this file
//! configuration = ["path/to/file", "conf_name"]
//! ```

use clap::Parser;
use clap::*;
use std::io::Read;
use std::io::Write;
use toml::Table;
//use HashMap
use std::collections::HashMap;
#[derive(Clone, Copy, PartialEq, Eq, Debug, ValueEnum)]
pub enum ParseMode {
    A,
    S,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ForceType {
    S,
    U,
    I,
    F,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineColumn {
    Line,
    Column,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OutputFormat {
    Csv,
    Txt,
    Exls,
}

#[derive(Debug, Clone, Copy, PartialEq)]
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

/// Commandline args
#[derive(Parser, Debug, PartialEq)]
#[command(version, about, long_about = None)]
pub struct Args {
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
    pub force_parse: Option<(Vec<(usize, ForceType)>, LineColumn)>,

    #[command(flatten)]
    pub output_settings: OutputSettings,

    #[arg(short = 'S', long, value_parser = validate_export_subtable)]
    /// Use a number or range end with `l/c` to specify the line or column
    /// Export the subtable of the cross parts of the lines and columns
    pub export_subtable: Option<(Vec<usize>, Vec<usize>)>,

    #[arg(short, long, requires = "config_name", value_hint = clap::ValueHint::FilePath)]
    /// Set the configuration file to use
    /// Use the configuration from the commandline first if conflict
    pub config: Option<std::path::PathBuf>,

    #[arg(short = 'n', long, requires = "config")]
    /// Set the configuration name you want to use in the configuration file
    pub config_name: Option<String>,

    #[arg(short, long)]
    /// Export the setting to the given toml file <DRY> , but not run the program
    pub dry: Option<String>,
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

impl Default for Args {
    fn default() -> Self {
        Args {
            input: None,
            seperation: " ".to_string(),
            end_line: "\n".to_string(),
            parse_mode: ParseMode::A,
            force_parse: None,
            output_settings: OutputSettings::default(),
            export_subtable: None,
            config: None,
            config_name: None,
            dry: None,
        }
    }
}

impl Args {
    pub fn from_toml(
        file: &str,
        name: &str,
        mut unique: Option<HashMap<(&str, &str), bool>>,
    ) -> Result<Args, std::io::Error> {
        if (unique.is_none()) {
            unique = Some(HashMap::new());
        } else if (unique.as_ref().unwrap().contains_key(&(file, name))) {
            panic!("Configuration file loop");
        }

        let unique = unique.map(|mut m| {
            m.insert((file, name), true);
            m
        });

        let content = std::fs::read(file)?;
        let s = std::str::from_utf8(&content).expect("Invalid UTF-8 sequence from toml file");
        let table = s.parse::<toml::Table>().expect("Invalid toml file");
        let conf = table
            .get(name)
            .expect("No such configuration in the toml file");

        // parse arguements
        let input = conf
            .get("input")
            .map(|path| std::path::PathBuf::from(path.as_str().expect("Invalid input path")));

        let seperation = conf
            .get("seperation")
            .map(|s| s.as_str().expect("Invalid seperation").to_string())
            .unwrap_or(" ".to_string());

        let end_line = conf
            .get("end_line")
            .map(|s| s.as_str().expect("Invalid end line").to_string())
            .unwrap_or("\n".to_string());

        let parse_mode = conf
            .get("is_auto")
            .map(|b| b.as_bool().expect("Invalid parse mode"));

        let mut force_parse: Option<(Vec<(usize, ForceType)>, LineColumn)> = None;
        let force = conf
            .get("force_parse")
            .map(|t| t.as_table().expect("Invalid force parse"));
        if force.is_some() {
            let force = force.unwrap();
            let l = force.get("line").is_some();
            let c = force.get("column").is_some();
            let force_array;
            let lc = if l && c {
                panic!("Can't set force parse for both line and column");
            } else if l {
                force_array = force
                    .get("line")
                    .map(|a| a.as_array().expect("Invalid force parse line"))
                    .unwrap();
                LineColumn::Line
            } else if c {
                force_array = force
                    .get("column")
                    .map(|a| a.as_array().expect("Invalid force parse column"))
                    .unwrap();
                LineColumn::Column
            } else {
                panic!("Invalid force parse");
            };
            force_parse = Some((Vec::new(), lc));
            for i in force_array {
                let i = i.as_array().expect("Invalid force parse");
                let start = i[0].as_integer().expect("Invalid force parse") as usize;
                let end = i[1].as_integer().expect("Invalid force parse") as usize;
                let t = i[2].as_str().expect("Invalid force parse");
                let t = match t {
                    "s" => ForceType::S,
                    "u" => ForceType::U,
                    "i" => ForceType::I,
                    "f" => ForceType::F,
                    _ => panic!("Invalid force parse"),
                };
                for j in start..=end {
                    force_parse.as_mut().unwrap().0.push((j, t));
                }
            }
        }

        let output = conf
            .get("export_path")
            .map(|s| s.as_str().expect("Invalid output path").to_string())
            .map(|v| {
                let suffix = v.split('.').last().expect("Invalid output path");
                match suffix {
                    "csv" => (v, OutputFormat::Csv),
                    "txt" => (v, OutputFormat::Txt),
                    "xlsx" | "xls" => (v, OutputFormat::Exls),
                    _ => panic!("Invalid output path"),
                }
            });

        let mut export_color = None;
        let color = conf
            .get("export_color")
            .map(|t| t.as_table().expect("Invalid export color"));
        if let Some(color) = color {
            let mut line: Vec<(usize, OutputColor)> = Vec::new();
            let mut column: Vec<(usize, OutputColor)> = Vec::new();
            if let Some(export_color_line) = color.get("line") {
                let export_color_line = export_color_line
                    .as_array()
                    .expect("Invalid export color line");
                for i in export_color_line {
                    let i = i.as_array().expect("Invalid export color line");
                    let start = i[0].as_integer().expect("Invalid export color line") as usize;
                    let end = i[1].as_integer().expect("Invalid export color line") as usize;
                    let c = i[2].as_str().expect("Invalid export color line");
                    let c = match c {
                        "r" => OutputColor::Red,
                        "g" => OutputColor::Green,
                        "b" => OutputColor::Blue,
                        "y" => OutputColor::Yellow,
                        "x" => OutputColor::Grey,
                        "w" => OutputColor::White,
                        _ => panic!("Invalid export color line"),
                    };
                    for j in start..=end {
                        line.push((j, c));
                    }
                }
            }
            if let Some(export_color_column) = color.get("column") {
                let export_color_column = export_color_column
                    .as_array()
                    .expect("Invalid export color column");
                for i in export_color_column {
                    let i = i.as_array().expect("Invalid export color column");
                    let start = i[0].as_integer().expect("Invalid export color column") as usize;
                    let end = i[1].as_integer().expect("Invalid export color column") as usize;
                    let c = i[2].as_str().expect("Invalid export color column");
                    let c = match c {
                        "r" => OutputColor::Red,
                        "g" => OutputColor::Green,
                        "b" => OutputColor::Blue,
                        "y" => OutputColor::Yellow,
                        "x" => OutputColor::Grey,
                        "w" => OutputColor::White,
                        _ => panic!("Invalid export color column"),
                    };
                    for j in start..=end {
                        column.push((j, c));
                    }
                }
            }
            export_color = Some((line, column));
        }

        let output_settings = OutputSettings {
            output,
            export_color,
        };

        let mut export_subtable = None;
        let export = conf
            .get("export_subtable")
            .map(|t| t.as_table().expect("Invalid export subtable"));
        let mut export_line = None;
        let mut export_column = None;
        if export.is_some() {
            let export = export.unwrap();
            export_line = export
                .get("line")
                .map(|t| t.as_array().expect("Invalid export subtable line"));
            export_column = export
                .get("column")
                .map(|t| t.as_array().expect("Invalid export subtable column"));
        }
        if export_line.is_some() || export_column.is_some() {
            let mut line = Vec::new();
            let mut column = Vec::new();
            if export_line.is_some() {
                let export_line = export_line.unwrap();
                for lines in export_line {
                    let lines = lines.as_array().expect("Invalid export subtable line");
                    let start =
                        lines[0].as_integer().expect("Invalid export subtable line") as usize;
                    let end = lines[1].as_integer().expect("Invalid export subtable line") as usize;
                    for i in start..=end {
                        line.push(i as usize);
                    }
                }
            }
            if export_column.is_some() {
                let export_column = export_column.unwrap();
                for columns in export_column {
                    let columns = columns.as_array().expect("Invalid export subtable column");
                    let start = columns[0]
                        .as_integer()
                        .expect("Invalid export subtable column")
                        as usize;
                    let end = columns[1]
                        .as_integer()
                        .expect("Invalid export subtable column")
                        as usize;
                    for i in start..=end {
                        column.push(i as usize);
                    }
                }
            }
            export_subtable = Some((line, column));
        }

        let (config, config_name) = if let Some(config) = conf.get("configuration") {
            let config = config.as_array().expect("Invalid configuration");
            let path = config[0].as_str().expect("Invalid configuration path");
            let name = config[1].as_str().expect("Invalid configuration name");
            (Some(std::path::PathBuf::from(path)), Some(name.to_string()))
        } else {
            (None, None)
        };

        let mut now_args = Args {
            input,
            seperation,
            end_line,
            parse_mode: if parse_mode.unwrap_or(true) {
                ParseMode::A
            } else {
                ParseMode::S
            },
            force_parse,
            output_settings,
            export_subtable,
            config,
            config_name,
            dry: None,
        };

        if (now_args.config.is_some() && now_args.config_name.is_some()) {
            let pre_settings = Self::from_toml(
                now_args.config.as_ref().unwrap().to_str().unwrap(),
                now_args.config_name.as_ref().unwrap(),
                unique.clone(),
            )
            .unwrap();
            if now_args.input == Args::default().input {
                now_args.input = pre_settings.input;
            }
            if now_args.seperation == Args::default().seperation {
                now_args.seperation = pre_settings.seperation;
            }
            if now_args.end_line == Args::default().end_line {
                now_args.end_line = pre_settings.end_line;
            }
            if now_args.parse_mode == Args::default().parse_mode {
                now_args.parse_mode = pre_settings.parse_mode;
            }
            if now_args.force_parse == Args::default().force_parse {
                now_args.force_parse = pre_settings.force_parse;
            }
            if now_args.output_settings.output == Args::default().output_settings.output {
                now_args.output_settings.output = pre_settings.output_settings.output;
            }
            if now_args.output_settings.export_color == Args::default().output_settings.export_color
            {
                now_args.output_settings.export_color = pre_settings.output_settings.export_color;
            }
            if now_args.export_subtable == Args::default().export_subtable {
                now_args.export_subtable = pre_settings.export_subtable;
            }
            if now_args.dry == Args::default().dry {
                now_args.dry = pre_settings.dry;
            }
            // config and config_name should not be kept as origin configuration file
        }

        return Ok(now_args);
    }
    pub fn to_toml(&self, _file: &str) -> Result<(), std::io::Error> {
        let mut file = std::fs::File::create(_file)?;

        //input
        let mut base_table = Table::new();
        base_table.insert(
            "input".to_owned(),
            toml::Value::String(self.input.as_ref().unwrap().to_str().unwrap().to_string()),
        );

        //seperation
        base_table.insert(
            "seperation".to_owned(),
            toml::Value::String(self.seperation.clone()),
        );

        //end_line
        if (self.end_line != "\n") {
            base_table.insert(
                "end_line".to_owned(),
                toml::Value::String(self.end_line.clone()),
            );
        }

        //is_auto
        base_table.insert(
            "is_auto".to_owned(),
            toml::Value::Boolean(self.parse_mode == ParseMode::A),
        );

        //export_subtable
        if let Some((line, column)) = &self.export_subtable {
            let mut subtable_table = Table::new();
            let mut line_table = Vec::new();
            let mut column_table = Vec::new();
            for l in line {
                let mut v = Vec::new();
                v.push(toml::Value::Integer(*l as i64));
                v.push(toml::Value::Integer(*l as i64));
                line_table.push(toml::Value::Array(v));
            }
            for c in column {
                let mut v = Vec::new();
                v.push(toml::Value::Integer(*c as i64));
                v.push(toml::Value::Integer(*c as i64));
                column_table.push(toml::Value::Array(v));
            }
            let mut is_empty = true;
            if (!line_table.is_empty()) {
                subtable_table.insert("line".to_owned(), toml::Value::Array(line_table));
                is_empty = false;
            }
            if (!column_table.is_empty()) {
                subtable_table.insert("column".to_owned(), toml::Value::Array(column_table));
                is_empty = false;
            }
            if (!is_empty) {
                base_table.insert(
                    "export_subtable".to_owned(),
                    toml::Value::Table(subtable_table),
                );
            }
        }

        //force_parse
        if let Some((force_parse, lc)) = &self.force_parse {
            let mut forsce_table = Table::new();
            let mut line = Vec::new();
            let mut column = Vec::new();
            for (i, t) in force_parse {
                let mut v = Vec::new();
                v.push(toml::Value::Integer(*i as i64));
                v.push(toml::Value::Integer(*i as i64));
                match t {
                    ForceType::S => v.push(toml::Value::String('s'.to_string())),
                    ForceType::U => v.push(toml::Value::String('u'.to_string())),
                    ForceType::I => v.push(toml::Value::String('i'.to_string())),
                    ForceType::F => v.push(toml::Value::String('f'.to_string())),
                }
                if *lc == LineColumn::Line {
                    line.push(toml::Value::Array(v));
                } else {
                    column.push(toml::Value::Array(v));
                }
            }
            if (!line.is_empty()) {
                forsce_table.insert("line".to_owned(), toml::Value::Array(line));
                base_table.insert("force_parse".to_owned(), toml::Value::Table(forsce_table));
            } else if (!column.is_empty()) {
                forsce_table.insert("column".to_owned(), toml::Value::Array(column));
                base_table.insert("force_parse".to_owned(), toml::Value::Table(forsce_table));
            }
        }

        //export_path
        if let Some((path, _)) = &self.output_settings.output {
            base_table.insert("export_path".to_owned(), toml::Value::String(path.clone()));
        }

        //export_color
        if let Some((line, column)) = &self.output_settings.export_color {
            let mut color_table = Table::new();
            let mut line_color = Vec::new();
            let mut column_color = Vec::new();
            for (i, c) in line {
                let mut v = Vec::new();
                //TODO:先把每行单独一个，不搞范围
                v.push(toml::Value::Integer(*i as i64));
                v.push(toml::Value::Integer(*i as i64));
                match c {
                    OutputColor::Black => v.push(toml::Value::String('b'.to_string())),
                    OutputColor::Red => v.push(toml::Value::String('r'.to_string())),
                    OutputColor::Green => v.push(toml::Value::String('g'.to_string())),
                    OutputColor::Blue => v.push(toml::Value::String('b'.to_string())),
                    OutputColor::Yellow => v.push(toml::Value::String('y'.to_string())),
                    OutputColor::Grey => v.push(toml::Value::String('x'.to_string())),
                    OutputColor::White => v.push(toml::Value::String('w'.to_string())),
                }
                line_color.push(toml::Value::Array(v));
            }
            for (i, c) in column {
                let mut v = Vec::new();
                v.push(toml::Value::Integer(*i as i64));
                v.push(toml::Value::Integer(*i as i64));
                match c {
                    OutputColor::Black => v.push(toml::Value::String('b'.to_string())),
                    OutputColor::Red => v.push(toml::Value::String('r'.to_string())),
                    OutputColor::Green => v.push(toml::Value::String('g'.to_string())),
                    OutputColor::Blue => v.push(toml::Value::String('b'.to_string())),
                    OutputColor::Yellow => v.push(toml::Value::String('y'.to_string())),
                    OutputColor::Grey => v.push(toml::Value::String('x'.to_string())),
                    OutputColor::White => v.push(toml::Value::String('w'.to_string())),
                }
                column_color.push(toml::Value::Array(v));
            }
            let mut is_empty = true;
            if (!line_color.is_empty()) {
                color_table.insert("line".to_owned(), toml::Value::Array(line_color));
                is_empty = false;
            }
            if (!column_color.is_empty()) {
                color_table.insert("column".to_owned(), toml::Value::Array(column_color));
                is_empty = false;
            }
            if (!is_empty) {
                base_table.insert("export_color".to_owned(), toml::Value::Table(color_table));
            }
        }

        //configuration
        {
            let mut tmp_config: Vec<toml::Value> = Vec::new();
            if self.config.is_some() {
                tmp_config.push(toml::Value::String(
                    self.config.as_ref().unwrap().to_str().unwrap().to_owned(),
                ));
                tmp_config.push(toml::Value::String(
                    self.config_name.as_ref().unwrap().to_owned(),
                ));
                base_table.insert("configuration".to_owned(), toml::Value::Array(tmp_config));
            }
        }

        let mut root_table = Table::new();
        root_table.insert("my_config".to_owned(), toml::Value::Table(base_table));
        file.write_all(&root_table.to_string().as_bytes())?;
        Ok(())
    }
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
                Some('c') => {
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
                Some('s') => t = ForceType::S,
                Some('u') => t = ForceType::U,
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
                Some('c') => {
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
                Some('s') => t = ForceType::S,
                Some('u') => t = ForceType::U,
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

mod read_tests {
    #[test]
    fn test_read_toml_simple() {
        let test1 =
            super::Args::from_toml("./tests/config/simple.toml", "simple_config1", None).unwrap();
        println!("test1 is\n {:#?}", test1);
        let test2 =
            super::Args::from_toml("./tests/config/simple.toml", "simple_config2", None).unwrap();
        println!("test2 is\n {:#?}", test2);
        let test3 =
            super::Args::from_toml("./tests/config/simple.toml", "simple_config3", None).unwrap();
        println!("test3 is\n {:#?}", test3);
    }

    #[test]
    fn test_read_toml_color() {
        let test1 =
            super::Args::from_toml("./tests/config/color.toml", "color_config1", None).unwrap();
        println!("color_test1 is\n {:#?}", test1);
        let test2 =
            super::Args::from_toml("./tests/config/color.toml", "color_config2", None).unwrap();
        println!("color_test2 is\n {:#?}", test2);
        let test3 =
            super::Args::from_toml("./tests/config/color.toml", "color_config3", None).unwrap();
        println!("color_test3 is\n {:#?}", test3);
    }

    #[test]
    fn test_read_toml_config() {
        let test1 = super::Args::from_toml(
            "./tests/config/configuration.toml",
            "configuration_config1",
            None,
        )
        .unwrap();
        println!("config_test1 is\n {:#?}", test1);
        let test2 = super::Args::from_toml(
            "./tests/config/configuration.toml",
            "configuration_config2",
            None,
        )
        .unwrap();
        println!("config_test2 is\n {:#?}", test2);
        let test3 = super::Args::from_toml(
            "./tests/config/configuration.toml",
            "configuration_config3",
            None,
        )
        .unwrap();
        println!("config_test3 is\n {:#?}", test3);
        let test4 = super::Args::from_toml(
            "./tests/config/configuration.toml",
            "configuration_config4",
            None,
        )
        .unwrap();
        println!("config_test4 is\n {:#?}", test3);
    }

    #[test]
    fn test_read_toml_excel() {
        let test1 =
            super::Args::from_toml("./tests/config/excel.toml", "default_config1", None).unwrap();
        println!("excel_test1 is\n {:#?}", test1);
        let test2 =
            super::Args::from_toml("./tests/config/excel.toml", "default_config2", None).unwrap();
        println!("excel_test2 is\n {:#?}", test2);
        let test3 =
            super::Args::from_toml("./tests/config/excel.toml", "default_config3", None).unwrap();
        println!("excel_test3 is\n {:#?}", test3);
    }
    #[test]
    fn test_read_toml_multiple() {
        let test1 =
            super::Args::from_toml("./tests/config/multiple.toml", "multiple_config1", None)
                .unwrap();
        println!("multiple_test1 is\n {:#?}", test1);
        let test2 =
            super::Args::from_toml("./tests/config/multiple.toml", "multiple_config2", None)
                .unwrap();
        println!("multiple_test2 is\n {:#?}", test2);
        let test3 =
            super::Args::from_toml("./tests/config/multiple.toml", "multiple_config3", None)
                .unwrap();
        println!("multiple_test3 is\n {:#?}", test3);
    }
    #[test]
    fn test_read_toml_subtable() {
        let test1 =
            super::Args::from_toml("./tests/config/subtable.toml", "subtable_config1", None)
                .unwrap();
        println!("subtable_test1 is\n {:#?}", test1);
        let test2 =
            super::Args::from_toml("./tests/config/subtable.toml", "subtable_config2", None)
                .unwrap();
        println!("subtable_test2 is\n {:#?}", test2);
        let test3 =
            super::Args::from_toml("./tests/config/subtable.toml", "subtable_config3", None)
                .unwrap();
        println!("subtable_test3 is\n {:#?}", test3);
    }
}

mod create_tests {

    #[test]
    fn test_create_toml_simpl() {
        let test3 =
            super::Args::from_toml("./tests/config/simple.toml", "simple_config3", None).unwrap();
        println!("simple_config3 is\n {:#?}", test3);
        test3.to_toml("./tests/create_config/simple3.toml").unwrap();
        let _test3 =
            super::Args::from_toml("./tests/create_config/simple3.toml", "my_config", None)
                .unwrap();
        assert_eq!(test3, _test3);
    }
    #[test]
    fn test_create_toml_config() {
        let test3 = super::Args::from_toml(
            "./tests/config/configuration.toml",
            "configuration_config3",
            None,
        )
        .unwrap();
        println!("configuration_config3 is\n {:#?}", test3);
        test3.to_toml("./tests/create_config/config3.toml").unwrap();
        let _test3 =
            super::Args::from_toml("./tests/create_config/config3.toml", "my_config", None)
                .unwrap();
        assert_eq!(test3, _test3);
    }

    #[test]
    fn test_create_toml_force() {
        let test3 =
            super::Args::from_toml("./tests/config/force.toml", "force_config2", None).unwrap();
        println!("force_config2 is\n {:#?}", test3);
        test3.to_toml("./tests/create_config/force3.toml").unwrap();
        let _test3 =
            super::Args::from_toml("./tests/create_config/force3.toml", "my_config", None).unwrap();
        assert_eq!(test3, _test3);
    }
    #[test]
    fn test_create_toml_color() {
        let test2 =
            super::Args::from_toml("./tests/config/color.toml", "color_config2", None).unwrap();
        println!("color_config2 is\n {:#?}", test2);
        test2.to_toml("./tests/create_config/color2.toml").unwrap();
        let _test2 =
            super::Args::from_toml("./tests/create_config/color2.toml", "my_config", None).unwrap();
        assert_eq!(test2, _test2);

        let test3 =
            super::Args::from_toml("./tests/config/color.toml", "color_config3", None).unwrap();
        println!("color_config3 is\n {:#?}", test3);
        test3.to_toml("./tests/create_config/color3.toml").unwrap();
        let _test3 =
            super::Args::from_toml("./tests/create_config/color3.toml", "my_config", None).unwrap();
        assert_eq!(test3, _test3);
    }
    #[test]
    fn test_create_toml_excel() {
        let test3 =
            super::Args::from_toml("./tests/config/excel.toml", "default_config3", None).unwrap();
        println!("default_config3 is\n {:#?}", test3);
        test3.to_toml("./tests/create_config/excel3.toml").unwrap();
        let _test3 =
            super::Args::from_toml("./tests/create_config/excel3.toml", "my_config", None).unwrap();
        assert_eq!(test3, _test3);
    }
    #[test]
    fn test_create_toml_subtable() {
        let test3 =
            super::Args::from_toml("./tests/config/subtable.toml", "subtable_config3", None)
                .unwrap();
        println!("subtable_config3 is\n {:#?}", test3);
        test3
            .to_toml("./tests/create_config/subtable3.toml")
            .unwrap();
        let _test3 =
            super::Args::from_toml("./tests/create_config/subtable3.toml", "my_config", None)
                .unwrap();
        assert_eq!(test3, _test3);
    }
    #[test]
    fn test_create_toml_multiple() {
        let test2 =
            super::Args::from_toml("./tests/config/multiple.toml", "multiple_config2", None)
                .unwrap();
        println!("multiple_config2 is\n {:#?}", test2);
        test2
            .to_toml("./tests/create_config/multiple2.toml")
            .unwrap();
        let _test2 =
            super::Args::from_toml("./tests/create_config/multiple2.toml", "my_config", None)
                .unwrap();
        assert_eq!(test2, _test2);

        let test3 =
            super::Args::from_toml("./tests/config/multiple.toml", "multiple_config3", None)
                .unwrap();
        println!("multiple_config3 is\n {:#?}", test3);
        test3
            .to_toml("./tests/create_config/multiple3.toml")
            .unwrap();
        let _test3 =
            super::Args::from_toml("./tests/create_config/multiple3.toml", "my_config", None)
                .unwrap();
        assert_eq!(test3, _test3);
    }
}
