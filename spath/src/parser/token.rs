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

use logos::Lexer;
use logos::Logos;

use crate::parser::error::Error;
use crate::parser::range::Range;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Token<'a> {
    pub source: &'a str,
    pub kind: TokenKind,
    pub span: Range,
}

impl fmt::Debug for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}({:?})", self.kind, self.span)
    }
}

impl<'a> Token<'a> {
    pub fn new_eoi(source: &'a str) -> Self {
        Token {
            source,
            kind: TokenKind::EOI,
            span: (source.len()..source.len()).into(),
        }
    }

    pub fn text(&self) -> &'a str {
        &self.source[std::ops::Range::from(self.span)]
    }
}

pub struct Tokenizer<'a> {
    source: &'a str,
    lexer: Lexer<'a, TokenKind>,
    eoi: bool,
}

impl<'a> Tokenizer<'a> {
    pub fn new(source: &'a str) -> Self {
        Tokenizer {
            source,
            lexer: TokenKind::lexer(source),
            eoi: false,
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Result<Token<'a>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.lexer.next() {
            Some(Err(_)) => {
                let span = Range::from(self.lexer.span().start..self.source.len());
                let message = "failed to recognize the rest tokens";
                Some(Err(Error::new(span, message)))
            }
            Some(Ok(kind)) => Some(Ok(Token {
                source: self.source,
                kind,
                span: self.lexer.span().into(),
            })),
            None => {
                if !self.eoi {
                    self.eoi = true;
                    Some(Ok(Token::new_eoi(self.source)))
                } else {
                    None
                }
            }
        }
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(logos::Logos, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TokenKind {
    EOI,

    #[regex(r"[ \t\r\n\f]+", logos::skip)]
    Whitespace,

    #[regex(r#"[_a-zA-Z\u0080-\uFFFF][_a-zA-Z0-9\u0080-\uFFFF]*"#)]
    Ident,

    #[regex(r#"'([^'\\]|\\.)*'"#)]
    #[regex(r#""([^"\\]|\\.)*""#)]
    LiteralString,

    #[regex(r"(-)?[0-9]+(_|[0-9])*")]
    LiteralInteger,

    #[regex(r"(-)?[0-9]+[eE][+-]?[0-9]+")]
    #[regex(r"(-)?[0-9]+\.[0-9]+([eE][+-]?[0-9]+)?")]
    LiteralFloat,

    // Symbols
    #[token("=")]
    #[token("==")]
    Eq,
    #[token("<>")]
    #[token("!=")]
    NotEq,
    #[token("!")]
    Not,
    #[token("<")]
    Lt,
    #[token(">")]
    Gt,
    #[token("<=")]
    Lte,
    #[token(">=")]
    Gte,
    #[token("&&")]
    And,
    #[token("||")]
    Or,
    #[token("$")]
    Dollar,
    #[token("@")]
    At,
    #[token(".")]
    Dot,
    #[token("*")]
    Asterisk,
    #[token(":")]
    Colon,
    #[token(",")]
    Comma,
    #[token("?")]
    QuestionMark,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,

    // §2.3.5.1. Syntax
    // true, false, and null are lowercase only (case-sensitive).
    #[token("false")]
    FALSE,
    #[token("null")]
    NULL,
    #[token("true")]
    TRUE,
}
