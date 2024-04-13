//! # Export
//! This module is trait used to export table, four ways will be supported:
//! 1. print to console with specific format
//! 2. write to txt with given format
//! 3. write to csv
//! 4. write to excel
//!
//! Table and Tableline implement this trait

pub trait Export {
    fn to_console(&self);
    fn to_txt(&self, file: &str, seperation: char) -> Result<(), String>;
    fn to_csv(&self, file: &str) -> Result<(), String>;
    fn to_excel(&self, file: &str, sheet: &str) -> Result<(), String>;
}
