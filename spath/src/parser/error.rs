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

use winnow::stream::Stream;

use crate::parser::input::Input;
use crate::parser::range::Range;

/// A parsing error with source span, message, and contexts.
#[derive(Debug)]
pub struct Error {
    range: Range,
    message: String,
    context: Vec<(Range, &'static str)>,
}

impl<'a, Registry> winnow::error::AddContext<Input<'a, Registry>> for Error {
    fn add_context(
        mut self,
        input: &Input<'a, Registry>,
        _token_start: &<Input<'a, Registry> as Stream>::Checkpoint,
        ctx: &'static str,
    ) -> Self {
        self.context.push((input[0].span, ctx));
        self
    }
}

impl<'a, Registry> winnow::error::ParserError<Input<'a, Registry>> for Error {
    type Inner = Self;

    fn from_input(input: &Input<'a, Registry>) -> Self {
        Self {
            range: input[0].span,
            message: "unexpected token".to_string(),
            context: vec![],
        }
    }

    fn into_inner(self) -> winnow::Result<Self::Inner, Self> {
        Ok(self)
    }
}

impl<'a, Registry, E> winnow::error::FromExternalError<Input<'a, Registry>, E> for Error
where
    E: std::fmt::Display,
{
    fn from_external_error(input: &Input<'a, Registry>, e: E) -> Self {
        Self {
            range: input[0].span,
            message: e.to_string(),
            context: vec![],
        }
    }
}

impl<'a, Registry> winnow::error::FromExternalError<Input<'a, Registry>, Error> for Error {
    fn from_external_error(_input: &Input<'a, Registry>, err: Error) -> Self {
        err
    }
}

impl Error {
    pub fn new(range: Range, message: impl Into<String>) -> Self {
        Self {
            range,
            message: message.into(),
            context: vec![],
        }
    }

    pub fn set_message(&mut self, message: impl Into<String>) {
        self.message = message.into();
    }

    pub fn to_pretty_string(&self, source: &str) -> String {
        use std::fmt::Write;

        let mut result = String::new();
        writeln!(&mut result, "failed to parse SPath expression:\n").unwrap();
        writeln!(&mut result, "{source}\n").unwrap();
        writeln!(&mut result, "InSpan {span}", span = self.range).unwrap();
        writeln!(&mut result, "Message: {message}", message = self.message).unwrap();
        for (range, ctx) in &self.context {
            writeln!(
                &mut result,
                "Context({range}): {ctx}",
                range = range,
                ctx = ctx
            )
            .unwrap();
        }
        result
    }
}
