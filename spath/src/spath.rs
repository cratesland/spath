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

use crate::spec::function::FunctionRegistry;
use crate::spec::query::Query;
use crate::spec::query::Queryable;
use crate::LocatedNodeList;
use crate::NodeList;
use crate::ParseError;
use crate::VariantValue;

#[derive(Debug, Clone)]
pub struct SPath<T: VariantValue, R: FunctionRegistry<Value = T>> {
    query: Query,
    registry: R,
}

impl<T: VariantValue, R: FunctionRegistry<Value = T>> SPath<T, R> {
    pub fn parse_with_registry(_query: &str, _registry: R) -> Result<Self, ParseError> {
        todo!()
    }

    pub fn query<'b>(&self, value: &'b T) -> NodeList<'b, T> {
        let nodes = self.query.query(value, value, &self.registry);
        NodeList::new(nodes)
    }

    pub fn query_located<'b>(&self, value: &'b T) -> LocatedNodeList<'b, T> {
        let nodes = self
            .query
            .query_located(value, value, &self.registry, Default::default());
        LocatedNodeList::new(nodes)
    }
}

impl<T: VariantValue, R: FunctionRegistry<Value = T>> fmt::Display for SPath<T, R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.query)
    }
}
