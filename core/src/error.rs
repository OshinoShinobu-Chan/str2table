/* Str2table core crate for error types
 * Copyright (C) 2024 Peng Zijun
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

//! Error types for Str2table
pub mod arg_error;
pub mod conflicts;
pub mod keyword_missing;
pub mod range_error;

/// A enum to describe the level of an error
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum ErrorLevel {
    /// Error that can be ignore or fixed automatically
    Warning,
    /// Error that can be fixed by user, e.g. recall the command
    Error,
    /// Error that caused by unrecovable reasons, e.g. file not found
    Fatal,
}

impl ToString for ErrorLevel {
    fn to_string(&self) -> String {
        match self {
            ErrorLevel::Warning => "[Warning]",
            ErrorLevel::Error => "[Error]",
            ErrorLevel::Fatal => "[Fatal]",
        }
        .to_string()
    }
}

/// A trait for Error types in this project
pub trait ErrorType: std::error::Error {
    /// Get the general description of this Error
    fn describe(&self) -> String;
    /// Get the level of this Error
    fn level(&self) -> ErrorLevel;
    /// Get the specific reason of this Error
    fn reason(&self) -> Option<String>;
    /// Get the attempt that program has already take to fix this error
    fn attempt(&self) -> Option<String>;
    /// Get the hint for user to fix this error
    fn hint(&self) -> Option<String>;
    /// Get the message to show on the screen
    fn message(&self, showing_level: ErrorLevel) -> String {
        let mut message = self.level().to_string();
        message.push_str(self.describe().as_str());

        if let Some(attempt) = self.attempt() {
            message.push_str(&format!(
                "\nProgram has tried this(these) attempt to fix this error:\n\t {}",
                attempt
            ));
        }
        if let Some(hint) = self.hint() {
            message.push_str(&format!(
                "\nYou may try the following method(s) to fix this:\n\t {} ",
                hint
            ));
        }
        if let Some(reason) = self.reason() {
            message.push_str(&format!(
                "\nThis error is caused by the following reason(s):\n {}",
                reason
            ));
        }
        if self.level() >= showing_level {
            message
        } else {
            "".to_string()
        }
    }
}
