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

use crate::spec::integer::Integer;
use crate::spec::query::Queryable;
use crate::ConcreteVariantArray;
use crate::LocatedNode;
use crate::NormalizedPath;
use crate::VariantValue;

/// For selecting array elements by their index.
///
/// Can use negative indices to index from the end of an array.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Index(pub Integer);

impl fmt::Display for Index {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Queryable for Index {
    fn query<'b, T: VariantValue>(&self, current: &'b T, _root: &'b T) -> Vec<&'b T> {
        if let Some(list) = current.as_array() {
            if self.0 < 0 {
                let abs = self.0.abs();
                usize::try_from(abs)
                    .ok()
                    .and_then(|i| list.len().checked_sub(i))
                    .and_then(|i| list.get(i))
                    .map(|v| vec![v])
                    .unwrap_or_default()
            } else {
                usize::try_from(self.0)
                    .ok()
                    .and_then(|i| list.get(i))
                    .map(|v| vec![v])
                    .unwrap_or_default()
            }
        } else {
            vec![]
        }
    }

    fn query_located<'b, T: VariantValue>(
        &self,
        current: &'b T,
        _root: &'b T,
        mut parent: NormalizedPath<'b>,
    ) -> Vec<LocatedNode<'b, T>> {
        if let Some((index, node)) = current.as_array().and_then(|list| {
            if self.0 < 0 {
                let abs = self.0.abs();
                usize::try_from(abs)
                    .ok()
                    .and_then(|i| list.len().checked_sub(i))
                    .and_then(|i| list.get(i).map(|v| (i, v)))
            } else {
                usize::try_from(self.0)
                    .ok()
                    .and_then(|i| list.get(i).map(|v| (i, v)))
            }
        }) {
            parent.push(index);
            vec![LocatedNode::new(parent, node)]
        } else {
            vec![]
        }
    }
}
