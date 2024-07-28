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

//! Conflicts is a error type that is used to describe the error of conflicts in commandlines
//! and maybe other places.
use super::{ErrorLevel, ErrorType};

pub struct Conflicts {
    pub name: String,
    pub description: String,
    pub level: ErrorLevel,
    pub reason: Option<String>,
    pub error_arg: Option<String>,
    pub whole_arg: Option<String>,
    pub error_location: Option<(usize, usize)>,
    pub hint: Option<String>,
}

impl Conflicts {
    pub fn new(
        level: ErrorLevel,
        error_arg: Option<String>,
        whole_arg: Option<String>,
        error_location: Option<(usize, usize)>,
        conflicts_vec: Option<Vec<String>>,
    ) -> Self {
        let mut flag = 0;
        if error_arg.is_some() {
            flag += 1;
        }
        if conflicts_vec.is_some() {
            flag += 2;
        }
        let name = "ConflictsError".to_string();
        let description = "This argument causes conflict(s).".to_string();
        let hint = Some("Please check the conflict(s) and try again.".to_string());
        match flag {
            0 => Self {
                name,
                description,
                level,
                reason: None,
                error_arg,
                whole_arg,
                error_location,
                hint,
            },
            1 => Self {
                name,
                description,
                level,
                reason: {
                    let error_arg = error_arg.clone().unwrap();
                    Some(format!("{error_arg} has conflicts in it.").to_string())
                },
                error_arg,
                whole_arg,
                error_location,
                hint,
            },
            2 => Self {
                name,
                description,
                level,
                reason: {
                    let conflicts_vec = conflicts_vec.unwrap();
                    Some(format!("{:?} conflict with each other.", conflicts_vec).to_string())
                },
                error_arg,
                whole_arg,
                error_location,
                hint,
            },
            3 => Self {
                name,
                description,
                level,
                reason: {
                    let error_arg = error_arg.clone().unwrap();
                    let conflicts_vec = conflicts_vec.unwrap();
                    Some(
                        format!(
                            "In {error_arg}, {:?} conflict with each other.",
                            conflicts_vec
                        )
                        .to_string(),
                    )
                },
                error_arg,
                whole_arg,
                error_location,
                hint,
            },
            _ => panic!("fatal error!"),
        }
    }
}

impl ErrorType for Conflicts {
    fn attempt(&self) -> Option<String> {
        None
    }
    fn describe(&self) -> String {
        self.description.clone()
    }
    fn hint(&self) -> Option<String> {
        self.hint.clone()
    }
    fn level(&self) -> ErrorLevel {
        self.level
    }
    fn reason(&self) -> Option<String> {
        self.reason.clone()
    }
}

impl std::error::Error for Conflicts {
    fn description(&self) -> &str {
        self.description.as_str()
    }
}

impl std::fmt::Display for Conflicts {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message(ErrorLevel::Error))
    }
}

impl std::fmt::Debug for Conflicts {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message(ErrorLevel::Error))
    }
}
