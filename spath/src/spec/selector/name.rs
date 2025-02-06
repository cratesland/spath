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

//! Name selectors for selecting object keys in SPath.

use std::fmt;

use crate::spec::functions::FunctionRegistry;
use crate::spec::query::Queryable;
use crate::ConcreteVariantObject;
use crate::LocatedNode;
use crate::NormalizedPath;
use crate::VariantValue;

/// §2.3.1 Name Selector.
///
/// Select a single variant object key.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Name {
    name: String,
}

impl Name {
    /// Create a new name selector.
    pub fn new(name: String) -> Self {
        Self { name }
    }

    /// Get as a string slice
    pub fn as_str(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "'{}'", self.name)
    }
}

impl Queryable for Name {
    fn query<'b, T: VariantValue, R: FunctionRegistry<Value = T>>(
        &self,
        current: &'b T,
        _root: &'b T,
        _registry: &R,
    ) -> Vec<&'b T> {
        let name = self.name.as_str();
        current
            .as_object()
            .and_then(|o| o.get(name))
            .map(|v| vec![v])
            .unwrap_or_default()
    }

    fn query_located<'b, T: VariantValue, R: FunctionRegistry<Value = T>>(
        &self,
        current: &'b T,
        _root: &'b T,
        _registry: &R,
        mut parent: NormalizedPath<'b>,
    ) -> Vec<LocatedNode<'b, T>> {
        let name = self.name.as_str();
        current
            .as_object()
            .and_then(|o| o.get_key_value(name))
            .map(|(k, v)| {
                parent.push(k);
                vec![LocatedNode::new(parent, v)]
            })
            .unwrap_or_default()
    }
}
