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

use std::fmt;

use winnow::token::literal;
use winnow::Parser;
use winnow::Stateful;

use crate::parser::error::Error;
use crate::parser::token::Token;
use crate::parser::token::TokenKind;
use crate::spec::function::FunctionRegistry;

pub struct InputState<Registry> {
    registry: Registry,
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

    pub fn registry(&self) -> &Registry {
        &self.registry
    }

    pub fn into_registry(self) -> Registry {
        self.registry
    }
}

pub type TokenSlice<'a> = winnow::stream::TokenSlice<'a, Token<'a>>;

pub type Input<'a, Registry> = Stateful<TokenSlice<'a>, InputState<Registry>>;

impl PartialEq<&str> for Token<'_> {
    fn eq(&self, other: &&str) -> bool {
        self.text() == *other
    }
}

impl PartialEq<TokenKind> for Token<'_> {
    fn eq(&self, other: &TokenKind) -> bool {
        self.kind == *other
    }
}

impl<'a, Registry> Parser<Input<'a, Registry>, &'a Token<'a>, Error> for TokenKind
where
    Registry: FunctionRegistry,
{
    fn parse_next(&mut self, input: &mut Input<'a, Registry>) -> Result<&'a Token<'a>, Error> {
        literal(*self).parse_next(input).map(|t| &t[0])
    }
}

pub fn text<'a, Registry>(
    text: &'static str,
) -> impl Parser<Input<'a, Registry>, &'a Token<'a>, Error>
where
    Registry: FunctionRegistry,
{
    move |input: &mut Input<'a, Registry>| literal(text).parse_next(input).map(|t| &t[0])
}
