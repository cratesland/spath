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

use std::str::FromStr;

use crate::parser::error::ParseError;
use crate::parser::expr::Expr;
use crate::parser::runner::run_parser;

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
