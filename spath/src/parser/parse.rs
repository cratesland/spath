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

use winnow::error::ContextError;

use crate::parser::input::TokenSlice;
use crate::spec::function::FunctionRegistry;
use crate::spec::query::Query;
use crate::VariantValue;

pub struct ParseContext<T: VariantValue, R: FunctionRegistry<Value = T>> {
    registry: R,
}

impl<T: VariantValue, R: FunctionRegistry<Value = T>> ParseContext<T, R> {
    pub fn new(registry: R) -> Self {
        Self { registry }
    }

    pub fn take_back_registry(self) -> R {
        self.registry
    }

    pub fn parse_query_main(&self, _input: TokenSlice) -> Result<Query, ContextError> {
        Ok(Query::default())
    }
}
