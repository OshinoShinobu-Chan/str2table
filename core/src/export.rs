/* Str2table core crate for export
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
//! # Export
//! This module is trait used to export table, four ways will be supported:
//! 1. print to console with specific format
//! 2. write to txt with given format
//! 3. write to csv
//! 4. write to excel
//!
//! Table and Tableline implement this trait

use xlsxwriter::prelude::*;

pub trait Export {
    fn to_console(&self);
    fn to_txt(&self, file: &str, seperation: char) -> Result<(), std::io::Error>;
    // fn to_csv(&self, file: &str) -> Result<(), String>;
    fn to_excel(&self, file: &str) -> Result<(), XlsxError>;
}
