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

use winnow::{Parser, Stateful};

use crate::parser::error::RefineError;
use crate::parser::input::text;
use crate::parser::input::TokenSlice;
use crate::parser::token::TokenKind::*;
use crate::spec::function::FunctionRegistry;
use crate::spec::query::Query;

pub type Input<'a, R> = Stateful<TokenSlice<'a>, ParseContext<R>>;

#[derive(Debug)]
pub struct ParseContext<R> {
    registry: R,
}

impl<R> ParseContext<R> {
    pub fn new(registry: R) -> Self {
        Self { registry }
    }

    pub fn take_back_registry(self) -> R {
        self.registry
    }
}

pub fn parse_query_main<R: FunctionRegistry>(input: &mut Input<R>) -> Result<Query, RefineError> {
    (parse_root_query, EOI)
        .map(|(query, _)| query)
        .parse_next(input)
}

fn parse_root_query<R: FunctionRegistry>(input: &mut Input<R>) -> Result<Query, RefineError> {
    text("$").map(|_| Query::default()).parse_next(input)
}
