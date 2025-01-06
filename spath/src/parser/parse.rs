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

use crate::parser::ast::Segment;
use crate::parser::ast::{Expr, Selector};
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

        // §2.2.1 (Root Identifier) Syntax
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

    fn parse_segment(&mut self) -> Result<Option<Segment>, ParseError> {
        let token = self.next_token();
        match token.kind {
            TokenKind::EOI => Ok(None),
            TokenKind::LBracket => {
                let selectors = self.parse_bracketed_selector()?;
                Ok(Some(Segment::Child { selectors }))
            }
            // TODO(tisonkun): handle ..(descendant segment), .identifier, and .*
            _ => Err(ParseError::unexpected_token(token.span)),
        }
    }

    fn parse_bracketed_selector(&mut self) -> Result<Vec<Selector>, ParseError> {
        let mut selectors = vec![];
        let mut prev_comma = false;
        loop {
            let token = self.peek_token();
            match token.kind {
                TokenKind::EOI | TokenKind::RBracket => {
                    let _consumed = self.next_token();
                    return Ok(selectors);
                }
                TokenKind::Comma => {
                    if prev_comma || selectors.is_empty() {
                        return Err(ParseError::unexpected_token(token.span));
                    }
                    prev_comma = true;
                    let _consumed = self.next_token();
                }
                _ => {
                    if prev_comma || selectors.is_empty() {
                        prev_comma = false;
                        let selector = self.parse_selector()?;
                        selectors.push(selector);
                    } else {
                        return Err(ParseError::unexpected_token(token.span));
                    }
                }
            }
        }
    }

    fn parse_selector(&mut self) -> Result<Selector, ParseError> {
        let token = self.next_token();
        match token.kind {
            TokenKind::Asterisk => Ok(Selector::Wildcard),
            TokenKind::LiteralString => {
                // TODO(tisonkun): unescape the string
                let text = token.text();
                Ok(Selector::Identifier {
                    name: text.to_string(),
                })
            }
            TokenKind::LiteralInteger => {
                // TODO(tisonkun): dispatch slice-selector
                let index = parse_integer(token)?;
                Ok(Selector::Index { index })
            }
            _ => Err(ParseError::unexpected_token(token.span)),
        }
    }

    fn peek_token(&self) -> Token {
        if self.index < self.tokens.len() {
            self.tokens[self.index]
        } else {
            Token::new_eoi(self.source)
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

fn parse_integer(token: Token) -> Result<i64, ParseError> {
    let text = token.text();
    text.parse()
        .map_err(|err| ParseError::new(token.span, format!("{err}")))
}
