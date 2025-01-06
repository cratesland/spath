use crate::parser::error::ParseError;
use crate::parser::range::Range;
use logos::{Lexer, Logos};
use std::fmt;

#[derive(Clone, PartialEq, Eq)]
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
    type Item = Result<Token<'a>, ParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.lexer.next() {
            Some(Err(_)) => {
                let span = Range::from(self.lexer.span().start..self.source.len());
                let message = "failed to recognize the rest tokens";
                Some(Err(ParseError::new(span, message)))
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

#[allow(non_camel_case_types)]
#[derive(logos::Logos, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TokenKind {
    EOI,

    #[regex(r"[ \t\r\n\f]+", logos::skip)]
    Whitespace,

    #[regex(r#"[_a-zA-Z][_a-zA-Z0-9]*"#)]
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

    // Keywords
    #[token("FALSE", ignore(ascii_case))]
    FALSE,
    #[token("NULL", ignore(ascii_case))]
    NULL,
    #[token("TRUE", ignore(ascii_case))]
    TRUE,
}
