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

use crate::parser::ast::Expr;
use crate::parser::ast::Segment;
use crate::parser::error::ParseError;
use crate::parser::token::Token;
use crate::parser::token::TokenKind;

#[derive(Debug)]
pub struct Parser<'a> {
    source: &'a str,
    tokens: Vec<Token<'a>>,
    /// The index of the first unprocessed token in `self.tokens`
    index: usize,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str, tokens: Vec<Token<'a>>) -> Self {
        Self {
            source,
            tokens,
            index: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Expr, ParseError> {
        let first = self.next_token();

        match first.kind {
            TokenKind::Dollar => {} // the first token must be '$'
            TokenKind::EOI => return Err(ParseError::empty()),
            _ => return Err(ParseError::unexpected_token(first.span)),
        }

        let mut segments = vec![];
        while let Some(segment) = self.parse_segment()? {
            segments.push(segment);
        }
        Ok(Expr::Segments { segments })
    }

    pub fn parse_segment(&mut self) -> Result<Option<Segment>, ParseError> {
        let token = self.next_token();
        match token.kind {
            TokenKind::EOI => Ok(None),
            _ => Err(ParseError::unexpected_token(token.span)),
        }
    }

    fn next_token(&mut self) -> Token {
        if self.index < self.tokens.len() {
            let token = self.tokens[self.index];
            self.index += 1;
            token
        } else {
            Token::new_eoi(self.source)
        }
    }
}
