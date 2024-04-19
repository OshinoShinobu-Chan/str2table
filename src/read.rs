//! # Read
//! This module used to read input from stdin or file, and parse it to table
use std::io::stdin;

use crate::table::Table;
/// Read a table from stdin with given seperation char
pub fn read_from_io(seperation: &str, end_line: &str) -> Table {
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
    Table::from_string(s, seperation, end_line)
}

/// Read a table from file with given seperation char
pub fn read_from_file(file: &str, seperation: &str, end_line: &str) -> Table {
    let s = std::fs::read_to_string(file).unwrap();
    Table::from_string(s, seperation, end_line)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_read_from_io() {
        let table = read_from_io(" ", "\n");
        println!("{:?}", table);
    }

    #[test]
    fn test_read_from_file() {
        let table = read_from_file("test.txt", " ", "\n");
        println!("{:?}", table);
    }
}
