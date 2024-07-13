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

pub mod core;
pub mod input;
pub mod output;

pub use core::*;
pub use input::*;
pub use output::*;

#[derive(Parser, Debug, PartialEq)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum Commands {
    #[clap(name = "input", about = "Parse a string to table", long_about = None)]
    Input(InputArgs),
    #[clap(name = "output", about = "Output a table to a file or console", long_about = None)]
    Output(OutputArgs),
}
