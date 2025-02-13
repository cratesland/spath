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

use crate::parser::error::Error;
use crate::parser::range::Range;
use crate::parser::token::Token;
use crate::parser::token::TokenKind;
use crate::spec::function::FunctionRegistry;
use std::fmt;
use std::sync::Arc;
use winnow::error::ParserError;
use winnow::stream::Stream;
use winnow::Parser;
use winnow::Stateful;

#[derive(Clone)]
pub struct InputState<Registry> {
    registry: Arc<Registry>,
}

impl<Registry> fmt::Debug for InputState<Registry> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("InputState").finish_non_exhaustive()
    }
}

impl<Registry> InputState<Registry> {
    pub fn new(registry: Registry) -> Self {
        Self { registry }
    }

    pub fn registry(&self) -> Arc<Registry> {
        self.registry.clone()
    }
}

pub type TokenSlice<'a> = winnow::stream::TokenSlice<'a, Token<'a>>;

pub type Input<'a, Registry> = Stateful<TokenSlice<'a>, InputState<Registry>>;

impl<'a, Registry> Parser<Input<'a, Registry>, &'a Token<'a>, Error> for TokenKind
where
    Registry: FunctionRegistry,
{
    fn parse_next(&mut self, input: &mut Input<'a, Registry>) -> Result<&'a Token<'a>, Error> {
        match input.first().filter(|token| token.kind == *self) {
            Some(_) => {
                // SAFETY: `first` returns `Some` if the input is not empty.
                let token = input.next_token().unwrap();
                Ok(token)
            }
            None => {
                if self.is_eoi() {
                    let start = input.first().unwrap().span.start;
                    let end = input.last().unwrap().span.end;
                    Err(Error::new_cut(
                        Range::from(start..end),
                        "failed to parse the rest of input",
                    ))
                } else {
                    let err = Error::from_input(input);
                    Err(err.with_message(format!("expected token {self:?}")))
                }
            }
        }
    }
}

pub fn text<'a, Registry>(
    text: &'static str,
) -> impl Parser<Input<'a, Registry>, &'a Token<'a>, Error>
where
    Registry: FunctionRegistry,
{
    move |input: &mut Input<'a, Registry>| match input.first().filter(|token| token.text() == text)
    {
        Some(_) => {
            // SAFETY: `first` returns `Some` if the input is not empty.
            let token = input.next_token().unwrap();
            Ok(token)
        }
        None => {
            let err = Error::from_input(input);
            Err(err.with_message(format!("expected text {text}")))
        }
    }
}
