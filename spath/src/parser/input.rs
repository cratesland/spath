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

use winnow::token::literal;
use winnow::Parser;

use crate::parser::error::RefineError;
use crate::parser::token::Token;
use crate::parser::token::TokenKind;

pub type TokenSlice<'a> = winnow::stream::TokenSlice<'a, Token<'a>>;

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

impl<'a> Parser<TokenSlice<'a>, &'a Token<'a>, RefineError> for TokenKind {
    fn parse_next(&mut self, input: &mut TokenSlice<'a>) -> Result<&'a Token<'a>, RefineError> {
        literal(*self).parse_next(input).map(|t| &t[0])
    }
}

pub fn text<'a>(text: &'static str) -> impl Parser<TokenSlice<'a>, &'a Token<'a>, RefineError> {
    move |input: &mut TokenSlice<'a>| literal(text).parse_next(input).map(|t| &t[0])
}
