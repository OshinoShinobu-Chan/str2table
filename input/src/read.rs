/*
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

//! # Read
//! This module used to read input from stdin or file, and parse it to table
use std::io::stdin;

use core::setting::Args;
use core::setting::ParseMode;
use core::table::Table;
/// Read a table from stdin with given seperation char
pub fn read_from_io(seperation: &str, end_line: &str, args: &Args) -> Table {
    let mut s = String::new();
    let lines = stdin().lines();
    for line in lines {
        if let Ok(line) = line {
            s.push_str(line.as_str());
        } else {
            break;
        }
        s.push('\n');
    }
    match args.parse_mode {
        ParseMode::A => Table::from_string(s, seperation, end_line),
        ParseMode::S => Table::from_string_force(s, seperation, end_line),
    }
}

/// Read a table from file with given seperation char
pub fn read_from_file(file: &str, seperation: &str, end_line: &str, args: &Args) -> Table {
    let s = std::fs::read_to_string(file).unwrap();
    match args.parse_mode {
        ParseMode::A => Table::from_string(s, seperation, end_line),
        ParseMode::S => Table::from_string_force(s, seperation, end_line),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // #[test]
    // fn test_read_from_io() {
    //     let table = read_from_io(" ", "\n");
    //     println!("{:?}", table);
    // }

    #[test]
    fn test_read_from_file() {
        let table = read_from_file("test.txt", " ", "\n", &Args::default());
        println!("{:?}", table);
    }
}
