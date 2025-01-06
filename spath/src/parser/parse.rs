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

use std::iter::Peekable;

use crate::parser::error::ParseError;
use crate::parser::expr::Expr;
use crate::parser::expr::Segment;
use crate::parser::expr::Selector;
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
            TokenKind::Dot => {
                let token = self.next_token();
                match token.kind {
                    TokenKind::Asterisk => Ok(Some(Segment::Child {
                        selectors: vec![Selector::Wildcard],
                    })),
                    TokenKind::Ident => {
                        let name = token.text().to_string();
                        Ok(Some(Segment::Descendant {
                            selectors: vec![Selector::Identifier { name }],
                        }))
                    }
                    TokenKind::Dot => self.parse_descendant_segment().map(Some),
                    _ => Err(ParseError::unexpected_token(token.span)),
                }
            }
            _ => Err(ParseError::unexpected_token(token.span)),
        }
    }

    fn parse_descendant_segment(&mut self) -> Result<Segment, ParseError> {
        let token = self.next_token();
        match token.kind {
            TokenKind::LBracket => {
                let selectors = self.parse_bracketed_selector()?;
                Ok(Segment::Descendant { selectors })
            }
            TokenKind::Asterisk => Ok(Segment::Descendant {
                selectors: vec![Selector::Wildcard],
            }),
            TokenKind::Ident => {
                let name = token.text().to_string();
                Ok(Segment::Descendant {
                    selectors: vec![Selector::Identifier { name }],
                })
            }
            _ => Err(ParseError::unexpected_token(token.span)),
        }
    }

    fn parse_bracketed_selector(&mut self) -> Result<Vec<Selector>, ParseError> {
        let mut selectors = vec![];
        let mut prev_comma = false;
        loop {
            let token = self.peek_token();
            match token.kind {
                TokenKind::EOI => {
                    return Err(ParseError::new(token.span, "unclosed bracket"));
                }
                TokenKind::RBracket => {
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
                let name = parse_string(token)?;
                Ok(Selector::Identifier { name })
            }
            TokenKind::LiteralInteger => {
                let index = parse_integer(token)?;
                if self.consume_token(TokenKind::Colon).is_some() {
                    self.parse_slice_selector(index)
                } else {
                    Ok(Selector::Index { index })
                }
            }
            TokenKind::Colon => self.parse_slice_selector(0),
            _ => Err(ParseError::unexpected_token(token.span)),
        }
    }

    fn parse_slice_selector(&mut self, start: i64) -> Result<Selector, ParseError> {
        let token = self.next_token();
        let mut end = None;
        let mut step = 1;
        match token.kind {
            TokenKind::Colon => {
                if let Some(token) = self.consume_token(TokenKind::LiteralInteger) {
                    // start::step
                    step = parse_integer(token)?;
                } // else - start::
            }
            TokenKind::LiteralInteger => {
                end = Some(parse_integer(token)?);
                if self.consume_token(TokenKind::Colon).is_some() {
                    if let Some(token) = self.consume_token(TokenKind::LiteralInteger) {
                        // start:end:step
                        step = parse_integer(token)?;
                    } // else - start:end:
                } // else - start:end
            }
            _ => return Err(ParseError::unexpected_token(token.span)),
        }
        Ok(Selector::Slice { start, end, step })
    }

    fn consume_token(&mut self, kind: TokenKind) -> Option<Token> {
        let token = self.peek_token();
        if token.kind == kind {
            Some(self.next_token())
        } else {
            None
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

fn parse_string(token: Token) -> Result<String, ParseError> {
    let text = token.text();
    let mut chars = text.chars();

    let quote = chars.next().expect("quote char always exist");
    if chars.next_back() != Some(quote) {
        return Err(ParseError::new(token.span, "mismatched quote"));
    }

    let mut chars = chars.peekable();
    let mut output = String::new();

    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('b') => output.push('\u{0008}'),
                Some('f') => output.push('\u{000C}'),
                Some('n') => output.push('\n'),
                Some('r') => output.push('\r'),
                Some('t') => output.push('\t'),
                Some('\\') => output.push('\\'),
                Some('u') => output.push(
                    unescape_unicode(&mut chars)
                        .ok_or_else(|| ParseError::new(token.span, "invalid escape sequence"))?,
                ),
                Some('x') => output.push(
                    unescape_byte(&mut chars)
                        .ok_or_else(|| ParseError::new(token.span, "invalid escape sequence"))?,
                ),
                Some(c) if c.is_digit(8) => output.push(unescape_octal(c, &mut chars)),
                Some(c) if c == quote => output.push(quote),
                _ => return Err(ParseError::new(token.span, "invalid escape sequence")),
            };
        } else if c == quote {
            return Err(ParseError::new(token.span, "intermediately close quote"));
        } else {
            output.push(c);
        }
    }

    Ok(output)
}

fn unescape_unicode(chars: &mut Peekable<impl Iterator<Item = char>>) -> Option<char> {
    let mut code = 0;

    for c in chars.take(4) {
        code = code * 16 + c.to_digit(16)?;
    }

    char::from_u32(code)
}

fn unescape_byte(chars: &mut Peekable<impl Iterator<Item = char>>) -> Option<char> {
    let mut byte = 0;

    for c in chars.take(2) {
        byte = byte * 16 + c.to_digit(16)?;
    }

    char::from_u32(byte)
}

fn unescape_octal(c1: char, chars: &mut Peekable<impl Iterator<Item = char>>) -> char {
    let mut oct = c1.to_digit(8).unwrap();

    while let Some(c) = chars.peek() {
        if let Some(digit) = c.to_digit(8) {
            oct = oct * 8 + digit;
            chars.next();
        } else {
            break;
        }
    }

    char::from_u32(oct).unwrap()
}
