# Str2table

A tool for linux command line written in Rust, to convert a character stream (a string) in a particular format to a data table.

## Usage
This module is used to set the setting of the table. Settings can be set by
commandline arguments temperarily or by a pre-set configuration file. You can
also mix the two ways to set the setting.
### Configuration Options
- `input`: The path of input file, use console input if not set
- `seperation`: The seperation char of the table
- `parse_mode`: Whether to parse the cell to auto type, or force to string
- `force_parse`: Force a line or a column or a cell to be parsed to a specific type
- `output`: The path of file to export the table, enable when export mode is not console
- `export_color`: Set the color of the table when export, by line or by column, enable when export mode is console
- `export_subtable`: Export a subtable of the table
### Commandline Options
- `-i` `<INPUT>`: Set the input path of the table as `<INPUT>`, use console input if not set
- `-s`/`--seperation` `<SEPERATION>`: Set the seperation pattern of the table as `<SEPERATION>`, default is ` `, can be multiple chars
- `-e`/`--end-line` `<END_LINE>`: Set the pattern to end the line as `<END_LINE>`, default is `\n`
- `-p`/`--parse-mode` `<PARSE_MODE>`: Set the parse mode of the table as `<PARSE_MODE>`, default is `a`(auto), can be `a` or `s`
- `-f`/`--force-parse` `<FORCE_PARSE>`: Force the lines or columns in `<FORCE_PARSE>` to be parsed as specific type.
Use number or range end with `l/c` to specify the line or column.
And only one number or range include `l/c` is ok.
Use `x-y` to specify the range, `x` and `y` are both included
Use `s/u/i/f` to specify the type, `s` for string, `i` for integer, `f` for float, at the end of every part.
Use `,` to seperate the lines or columns, and do not use space
Panic if the the force type is conflict.
Panic if `l` and `c` are both used in this arguement.
If the force type has error, then use auto_parse.
Lines or columns that do not exist will be ignored.
- `-o`/`--output` `<OUTPUT>`: Set the path of file to export the table as `<OUTPUT>`, enable when export mode is not console.
Infer the format by the suffix of the file, support `csv`, `txt`, `exls`.
- `-C`/`--export-color` `<EXPORT_COLOR>`: Set the color of the table by line, enable when export mode is console
Use number or range end with `l/c` and with color, default is black.
`r` represents red, `g` represents green, `b` represents blue, `y` represents yellow, `x` represents grey
`w` represents white.
Follow the line color first if conflict.
- `-S`/`--export-subtable` `<EXPORT_SUBTABLE>`: Set the subtable to export, default is the whole table.
Use number or range end with `l/c` to specify the line or column.
Export the subtable of the cross parts of the lines and columns.
- `-c`/`--config` `<EXPORT_PATH>`: Set the configuration file to use as `<EXPORT_PATH>`.
Use the configuration from the commandline first if conflict.
- `-n`/`--config-name` `<EXPORT_NAME>`: Set the configuration name you want to use in the configuration file as `<EXPORT_NAME>`.
- `-d`/`--dry` `<DRY>` : Export the setting to the given toml file `<DRY>` , but not run the program.
- `-h`/`--help`: Print the help message.
#### Example
```bash
str2table -s '#' -pm s -fp 1-2li,4f -ecl 1lr,2lg,3cb -es 1-3l,1-3c
```
This command means, read a table from console with `#` as seperation char,
parse the table to string, force the first two lines to be integer, and fourth lines to be float
export the table to concole`, set the color
of the first line to red, the second line to green, the third column to blue,
export the subtable of the first three lines and the first three columns.
### Configuration File
The configuration file is a toml file, with the following format:
```toml
# Configuration file for str2table
# You can use conf_name to set the name of the configuration
# So you can include multiple configuration in one file
[conf_name]
# input path, use console input if not set
input = "input.txt"
# seperation char, default is ' '
seperation = "#"
# Is auto parse, default is true
# if set to false, force all the data to str
is_auto = true
# force parse line, use an array, default is []
# the following example means, force the first line to string,
# the second line to fourth line to integer
force_parse.line = [
[1, 1, 's'],
[2, 4, 'i'],
]
# force parse column, use an array, default is [], same as line
# this can't be used with force_parse.line
# force_parse.column = [
# [1, 1, 's'],
# [2, 2, 'i'],
# ]
# export path, use console output if not set
export_path = "output.txt"
# export color by line, use an array, default is []
# the following example means, set the first line to red,
# the second line to fourth line to green, the third line to blue
export_color.line = [
[1, 1, 'r'],
[2, 4, 'g'],
]
# export color by column, use an array, default is [], same as line
export_color.column = [
[1, 1, 'r'],
[2, 2, 'g'],
]
# export subtable line, use an array, default export the whole line
# you can also use an array of two to represent a range
# the following example means, export the first line and third line
export_subtable.line = [[1, 1] , [3, 3] ]
# export subtable column, use an array, default export the whole column
# you can also use an array of two to represent a range
# the following example means, export the first to second columns and fourth column
export_subtable.column = [[1, 2], [4, 4]]
# use configuration from other configuration module, use config from this configuration first if conflict
# if you use . as path, then find the conf_name in this file
configuration = ["path/to/file", "conf_name"]
```