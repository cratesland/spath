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

//! Index selectors in SPath.

use std::fmt;

use num_traits::ToPrimitive;

use crate::spec::functions::FunctionRegistry;
use crate::spec::query::Queryable;
use crate::ConcreteVariantArray;
use crate::LocatedNode;
use crate::NormalizedPath;
use crate::VariantValue;

/// §2.3.3 Index Selector.
///
/// For selecting array elements by their index.
///
/// Can use negative indices to index from the end of an array.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Index {
    /// The index of the selector.
    index: i64,
}

impl Index {
    /// Create a new index selector.
    pub fn new(index: i64) -> Self {
        Self { index }
    }

    /// Get the index of the selector.
    pub fn index(&self) -> i64 {
        self.index
    }
}

impl fmt::Display for Index {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.index)
    }
}

// §2.3.3.2. (Index Selector) Semantics
fn resolve_index(index: i64, len: usize) -> Option<usize> {
    let index = if index >= 0 {
        index.to_usize()?
    } else {
        let index = len.to_i64().unwrap_or(i64::MAX) + index;
        index.to_usize()?
    };

    if index < len {
        Some(index)
    } else {
        None
    }
}

impl Queryable for Index {
    fn query<'b, T: VariantValue, R: FunctionRegistry<Value = T>>(
        &self,
        current: &'b T,
        _root: &'b T,
        _registry: &R,
    ) -> Vec<&'b T> {
        current
            .as_array()
            .and_then(|list| {
                let index = resolve_index(self.index, list.len())?;
                list.get(index)
            })
            .map(|node| vec![node])
            .unwrap_or_default()
    }

    fn query_located<'b, T: VariantValue, R: FunctionRegistry<Value = T>>(
        &self,
        current: &'b T,
        _root: &'b T,
        _registry: &R,
        mut parent: NormalizedPath<'b>,
    ) -> Vec<LocatedNode<'b, T>> {
        current
            .as_array()
            .and_then(|list| {
                let index = resolve_index(self.index, list.len())?;
                list.get(index).map(|node| (index, node))
            })
            .map(|(i, node)| {
                parent.push(i);
                vec![LocatedNode::new(parent, node)]
            })
            .unwrap_or_default()
    }
}
