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

//! RangeError is a error type that is used to describe the error in the range of a value.
use super::{ErrorLevel, ErrorType};

pub enum RangeErrorKind {
    OutOfRange,
    LeftSideError,
    RightSideError,
    BothSidesError,
    SingleNumberError,
}

impl ToString for RangeErrorKind {
    fn to_string(&self) -> String {
        match self {
            RangeErrorKind::OutOfRange => "OutOfRange".to_string(),
            RangeErrorKind::LeftSideError => "LeftSideError".to_string(),
            RangeErrorKind::RightSideError => "RightSideError".to_string(),
            RangeErrorKind::BothSidesError => "BothSidesError".to_string(),
            RangeErrorKind::SingleNumberError => "SingleNumberError".to_string(),
        }
    }
}

impl RangeErrorKind {
    pub fn get_reason(&self) -> String {
        match self {
            RangeErrorKind::OutOfRange => "the value is out of the range.".to_string(),
            RangeErrorKind::LeftSideError => {
                "the left side of the range is missing or not a number.".to_string()
            }
            RangeErrorKind::RightSideError => {
                "the right side of the range is missing or not a number.".to_string()
            }
            RangeErrorKind::BothSidesError => {
                "both sides of the range are missing or not a number.".to_string()
            }
            RangeErrorKind::SingleNumberError => {
                "the single number is missing or not a number.".to_string()
            }
        }
    }
}

pub struct RangeError {
    pub name: String,
    pub description: String,
    pub level: ErrorLevel,
    pub reason: Option<String>,
    pub error_value: Option<String>,
    pub whole_value: Option<String>,
    pub error_location: Option<(usize, usize)>,
    pub hint: Option<String>,
}

impl RangeError {
    pub fn new(
        error_type: RangeErrorKind,
        error_value: Option<String>,
        whole_value: Option<String>,
        error_location: Option<(usize, usize)>,
    ) -> Self {
        Self {
            name: "RangeError".to_string(),
            description: error_type.to_string(),
            level: ErrorLevel::Error,
            reason: Some(error_type.get_reason()),
            error_value,
            whole_value,
            error_location,
            hint: Some("Please check the ragne again.".to_string()),
        }
    }
}

impl ErrorType for RangeError {
    fn describe(&self) -> String {
        self.description.clone()
    }

    fn level(&self) -> ErrorLevel {
        self.level
    }

    fn reason(&self) -> Option<String> {
        if self.reason.is_none() {
            return None;
        }
        let mut reason = "Error happens in \"".to_string();
        reason.push_str(self.error_value.as_ref().unwrap().as_str());
        reason.push_str("\", where ");
        reason.push_str(self.reason.as_ref().unwrap().as_str());
        Some(reason)
    }

    fn attempt(&self) -> Option<String> {
        None
    }

    fn hint(&self) -> Option<String> {
        self.hint.clone()
    }
}

impl std::error::Error for RangeError {
    fn description(&self) -> &str {
        self.description.as_str()
    }
}

impl std::fmt::Display for RangeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message(ErrorLevel::Warning))
    }
}

impl std::fmt::Debug for RangeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message(ErrorLevel::Warning))
    }
}
