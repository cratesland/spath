// Copyright 2024 tison <wander4096@gmail.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::parser::input::Input;
use crate::parser::range::Range;
use crate::spec::function::FunctionValidationError;
use crate::spec::selector::filter::NonSingularQueryError;
use crate::ParseError;
use std::fmt::Debug;
use winnow::error::{FromExternalError, ModalError, Needed, ParserError};

/// An in-flight parsing error.
#[derive(Debug)]
pub struct Error {
    range: Range,
    message: String,
    cut: bool,
}

impl<'a, Registry> ParserError<Input<'a, Registry>> for Error {
    type Inner = Self;

    fn from_input(input: &Input<'a, Registry>) -> Self {
        Self {
            range: input[0].span,
            message: "unexpected token".to_string(),
            cut: false,
        }
    }

    fn assert(input: &Input<'a, Registry>, message: &'static str) -> Self
    where
        Input<'a, Registry>: Debug,
    {
        Self {
            range: input[0].span,
            message: message.to_string(),
            cut: true,
        }
    }

    fn incomplete(_input: &Input<'a, Registry>, _needed: Needed) -> Self {
        unreachable!("this parser is not partial")
    }

    fn or(self, other: Self) -> Self {
        if self.cut {
            self
        } else {
            other
        }
    }

    fn is_backtrack(&self) -> bool {
        !self.cut
    }

    fn into_inner(self) -> winnow::Result<Self::Inner, Self> {
        Ok(self)
    }
}

impl<'a, Registry> FromExternalError<Input<'a, Registry>, Error> for Error {
    fn from_external_error(_input: &Input<'a, Registry>, err: Error) -> Self {
        err
    }
}

impl<'a, Registry> FromExternalError<Input<'a, Registry>, FunctionValidationError> for Error {
    fn from_external_error(input: &Input<'a, Registry>, err: FunctionValidationError) -> Self {
        let range = input[0].span;
        let message = format!("{err}");
        Self {
            range,
            message,
            cut: true,
        }
    }
}

impl<'a, Registry> FromExternalError<Input<'a, Registry>, NonSingularQueryError> for Error {
    fn from_external_error(input: &Input<'a, Registry>, err: NonSingularQueryError) -> Self {
        let range = input[0].span;
        let message = format!("{err}");
        Self {
            range,
            message,
            cut: false,
        }
    }
}

impl ModalError for Error {
    fn cut(mut self) -> Self {
        self.cut = true;
        self
    }

    fn backtrack(mut self) -> Self {
        self.cut = false;
        self
    }
}

impl Error {
    pub fn new_cut(range: Range, message: impl Into<String>) -> Self {
        Self {
            range,
            message: message.into(),
            cut: true,
        }
    }

    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = message.into();
        self
    }

    pub fn into_parse_error(self, source: impl Into<String>) -> ParseError {
        ParseError {
            source: source.into(),
            range: self.range.into(),
            message: self.message,
        }
    }
}
