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

//! Types representing the different selectors in SPath.

pub mod filter;
pub mod index;
pub mod name;
pub mod slice;

use std::fmt;

use self::index::Index;
use self::name::Name;
use self::slice::Slice;
use crate::spec::query::Queryable;
use crate::spec::select_wildcard;
use crate::spec::selector::filter::Filter;
use crate::ConcreteVariantArray;
use crate::ConcreteVariantObject;
use crate::LocatedNode;
use crate::NormalizedPath;
use crate::VariantValue;

/// An SPath selector
#[derive(Debug, Clone)]
pub enum Selector {
    /// Select an object key
    Name(Name),
    /// Select all nodes
    ///
    /// For an object, this produces a nodelist of all member values; for an array, this produces a
    /// nodelist of all array elements.
    Wildcard,
    /// Select an array element
    Index(Index),
    /// Select a slice from an array
    ArraySlice(Slice),
    /// Use a filter to select nodes
    Filter(Filter),
}

impl Selector {
    /// Will the selector select at most only a single node
    pub fn is_singular(&self) -> bool {
        matches!(self, Selector::Name(_) | Selector::Index(_))
    }
}

impl fmt::Display for Selector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Selector::Name(name) => write!(f, "{name}"),
            Selector::Wildcard => write!(f, "*"),
            Selector::Index(index) => write!(f, "{index}"),
            Selector::ArraySlice(slice) => write!(f, "{slice}"),
            Selector::Filter(filter) => write!(f, "?{filter}"),
        }
    }
}

impl Queryable for Selector {
    fn query<'b, T: VariantValue>(&self, current: &'b T, root: &'b T) -> Vec<&'b T> {
        let mut result = Vec::new();
        match self {
            Selector::Name(name) => result.append(&mut name.query(current, root)),
            Selector::Wildcard => select_wildcard(&mut result, current),
            Selector::Index(index) => result.append(&mut index.query(current, root)),
            Selector::ArraySlice(slice) => result.append(&mut slice.query(current, root)),
            Selector::Filter(filter) => result.append(&mut filter.query(current, root)),
        }
        result
    }

    fn query_located<'b, T: VariantValue>(
        &self,
        current: &'b T,
        root: &'b T,
        parent: NormalizedPath<'b>,
    ) -> Vec<LocatedNode<'b, T>> {
        match self {
            Selector::Name(name) => name.query_located(current, root, parent),
            Selector::Wildcard => {
                if let Some(list) = current.as_array() {
                    list.iter()
                        .enumerate()
                        .map(|(i, node)| LocatedNode::new(parent.clone_and_push(i), node))
                        .collect()
                } else if let Some(obj) = current.as_object() {
                    obj.iter()
                        .map(|(k, node)| LocatedNode::new(parent.clone_and_push(k), node))
                        .collect()
                } else {
                    vec![]
                }
            }
            Selector::Index(index) => index.query_located(current, root, parent),
            Selector::ArraySlice(slice) => slice.query_located(current, root, parent),
            Selector::Filter(filter) => filter.query_located(current, root, parent),
        }
    }
}
