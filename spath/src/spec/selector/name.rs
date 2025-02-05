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

use crate::spec::query::Queryable;
use crate::ConcreteVariantObject;
use crate::LocatedNode;
use crate::NormalizedPath;
use crate::VariantValue;

/// Select a single variant object key.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Name(pub String);

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "'{name}'", name = self.0)
    }
}

impl Queryable for Name {
    fn query<'b, T: VariantValue>(&self, current: &'b T, _root: &'b T) -> Vec<&'b T> {
        current
            .as_object()
            .and_then(|o| o.get(&self.0))
            .map(|v| vec![v])
            .unwrap_or_default()
    }

    fn query_located<'b, T: VariantValue>(
        &self,
        current: &'b T,
        _root: &'b T,
        mut parent: NormalizedPath<'b>,
    ) -> Vec<LocatedNode<'b, T>> {
        current
            .as_object()
            .and_then(|o| o.get_key_value(&self.0))
            .map(|(k, v)| {
                parent.push(k);
                vec![LocatedNode::new(parent, v)]
            })
            .unwrap_or_default()
    }
}

impl From<&str> for Name {
    fn from(s: &str) -> Self {
        Self(s.to_owned())
    }
}
