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

//! ArgError is a error type that is used to describe the error in the commandline arguments.
use super::{ErrorLevel, ErrorType};

pub struct ArgError {
    pub name: String,
    pub description: String,
    pub level: ErrorLevel,
    pub reason: Option<String>,
    pub error_arg: Option<String>,
    pub whole_arg: Option<String>,
    pub error_location: Option<(usize, usize)>,
    pub hint: Option<String>,
}

pub enum ArgErrorKind {
    NoImplementation,
    WrongFormat,
    Conflicts,
    FormatError,
}

impl ToString for ArgErrorKind {
    fn to_string(&self) -> String {
        match self {
            ArgErrorKind::NoImplementation => "NoImplementation".to_string(),
            ArgErrorKind::WrongFormat => "WrongFormat".to_string(),
            ArgErrorKind::Conflicts => "Conflicts".to_string(),
            ArgErrorKind::FormatError => "FormatError".to_string(),
        }
    }
}

impl ArgErrorKind {
    pub fn get_description(&self) -> String {
        match self {
            ArgErrorKind::NoImplementation => "This argument is not implemented fully.".to_string(),
            ArgErrorKind::WrongFormat => "The format of this argument is wrong.".to_string(),
            ArgErrorKind::Conflicts => "This argument causes conflict(s)".to_string(),
            ArgErrorKind::FormatError => "This file format is unsupported.".to_string(),
        }
    }
    pub fn get_hint(&self) -> Option<String> {
        match self {
            ArgErrorKind::NoImplementation => Some("Please wait for the next version.".to_string()),
            ArgErrorKind::WrongFormat => {
                Some("Please check the format of this argument.".to_string())
            }
            ArgErrorKind::Conflicts => Some("Please check the reason.".to_string()),
            ArgErrorKind::FormatError => Some("Please check the format of the file.".to_string()),
        }
    }
    pub fn get_level(&self) -> ErrorLevel {
        match self {
            ArgErrorKind::NoImplementation => ErrorLevel::Warning,
            ArgErrorKind::WrongFormat => ErrorLevel::Error,
            ArgErrorKind::Conflicts => ErrorLevel::Error,
            ArgErrorKind::FormatError => ErrorLevel::Error,
        }
    }
}

impl ArgError {
    pub fn new(
        error_type: ArgErrorKind,
        reason: Option<String>,
        error_arg: Option<String>,
        whole_arg: Option<String>,
        error_location: Option<(usize, usize)>,
        hint: Option<String>,
    ) -> ArgError {
        let name = error_type.to_string();
        let description = error_type.get_description();
        let hint_ = error_type.get_hint();
        let level = error_type.get_level();
        ArgError {
            name,
            description,
            level,
            reason,
            error_arg,
            whole_arg,
            error_location,
            hint: if hint.is_none() { hint } else { hint_ },
        }
    }
}

impl ErrorType for ArgError {
    fn describe(&self) -> String {
        self.description.clone()
    }
    fn attempt(&self) -> Option<String> {
        None
    }
    fn reason(&self) -> Option<String> {
        if self.reason.is_none() {
            return None;
        }
        let mut reason = "Error happens in \"".to_string();
        reason.push_str(self.whole_arg.as_ref().unwrap().as_str());
        reason.push_str("\", where");
        if self.error_arg.is_some() {
            reason.push_str(" \"");
            reason.push_str(self.error_arg.as_ref().unwrap().as_str());
            reason.push_str("\" ");
        }
        reason.push_str(self.reason.as_ref().unwrap().as_str());
        Some(reason)
    }
    fn hint(&self) -> Option<String> {
        self.hint.clone()
    }
    fn level(&self) -> ErrorLevel {
        self.level
    }
}

impl std::error::Error for ArgError {
    fn description(&self) -> &str {
        self.description.as_str()
    }
}

impl std::fmt::Display for ArgError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message(ErrorLevel::Warning))
    }
}

impl std::fmt::Debug for ArgError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message(ErrorLevel::Warning))
    }
}
