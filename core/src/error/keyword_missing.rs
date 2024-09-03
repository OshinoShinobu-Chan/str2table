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

//! keyword_missing is a error type that is used to describe the error in the keyword missing.
use super::{ErrorLevel, ErrorType};

pub struct KeywordMissing {
    pub name: String,
    pub description: String,
    pub level: ErrorLevel,
    pub reason: Option<String>,
    pub error_arg: Option<String>,
    pub whole_arg: Option<String>,
    pub error_location: Option<(usize, usize)>,
    pub hint: Option<String>,
}

impl KeywordMissing {
    pub fn new(
        error_arg: Option<String>,
        whole_arg: Option<String>,
        error_location: Option<(usize, usize)>,
        keyword: String,
    ) -> Self {
        let name = "KeywordError".to_string();
        let description = format!("{} is missing or wrong.", keyword);
        let hint = Some("Please check the keyword and try again.".to_string());
        let level = ErrorLevel::Error;
        let reason = if error_arg.is_none() {
            None
        } else {
            Some(format!(
                "In {} where {} is expected.",
                whole_arg.as_ref().unwrap(),
                keyword
            ))
        };
        Self {
            name,
            description,
            level,
            reason,
            error_arg,
            whole_arg,
            error_location,
            hint,
        }
    }
}

impl ErrorType for KeywordMissing {
    fn attempt(&self) -> Option<String> {
        None
    }
    fn describe(&self) -> String {
        self.description.clone()
    }

    fn level(&self) -> ErrorLevel {
        self.level
    }

    fn reason(&self) -> Option<String> {
        self.reason.clone()
    }

    fn hint(&self) -> Option<String> {
        self.hint.clone()
    }
}

impl std::error::Error for KeywordMissing {
    fn description(&self) -> &str {
        self.description.as_str()
    }
}

impl std::fmt::Display for KeywordMissing {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message(ErrorLevel::Error))
    }
}

impl std::fmt::Debug for KeywordMissing {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message(ErrorLevel::Error))
    }
}
