use crate::parser::ast::Expr;
use crate::parser::error::ParseError;
use crate::parser::runner::run_parser;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct SPath {
    pub expr: Expr,
}

impl FromStr for SPath {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let expr = run_parser(s)?;
        Ok(SPath { expr })
    }
}
