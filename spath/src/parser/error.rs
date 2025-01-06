use crate::parser::range::Range;

#[derive(Debug, thiserror::Error)]
#[error("{message}")]
pub struct ParseError {
    span: Range,
    message: String,
}

impl ParseError {
    pub fn new(span: Range, message: impl Into<String>) -> Self {
        Self {
            span,
            message: message.into(),
        }
    }
}
