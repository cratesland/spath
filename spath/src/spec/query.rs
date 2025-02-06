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

//! Types representing queries in SPath

use std::fmt;

use super::segment::QuerySegment;
use crate::node::LocatedNode;
use crate::path::NormalizedPath;
use crate::VariantValue;

mod sealed {
    use super::Query;
    use crate::spec::segment::QuerySegment;
    use crate::spec::segment::Segment;
    use crate::spec::selector::filter::Filter;
    use crate::spec::selector::index::Index;
    use crate::spec::selector::name::Name;
    use crate::spec::selector::slice::Slice;
    use crate::spec::selector::Selector;

    pub trait Sealed {}
    impl Sealed for Query {}
    impl Sealed for QuerySegment {}
    impl Sealed for Segment {}
    impl Sealed for Slice {}
    impl Sealed for Name {}
    impl Sealed for Selector {}
    impl Sealed for Index {}
    impl Sealed for Filter {}
}

/// A trait that can query a variant value.
pub trait Queryable: sealed::Sealed {
    /// Run the query over a `current` node with a `root` node.
    fn query<'b, T: VariantValue>(&self, current: &'b T, root: &'b T) -> Vec<&'b T>;

    /// Run the query over a `current` node with a `root` node and a `parent` path.
    fn query_located<'b, T: VariantValue>(
        &self,
        current: &'b T,
        root: &'b T,
        parent: NormalizedPath<'b>,
    ) -> Vec<LocatedNode<'b, T>>;
}

/// Represents a SPath expression
#[derive(Debug, Clone, Default)]
pub struct Query {
    /// The kind of query, root (`$`), or current (`@`)
    pub kind: QueryKind,
    /// The segments constituting the query
    pub segments: Vec<QuerySegment>,
}

impl Query {
    pub(crate) fn is_singular(&self) -> bool {
        for s in &self.segments {
            if s.is_descendent() {
                return false;
            }
            if !s.segment.is_singular() {
                return false;
            }
        }
        true
    }
}

impl fmt::Display for Query {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            QueryKind::Root => write!(f, "$")?,
            QueryKind::Current => write!(f, "@")?,
        }
        for s in &self.segments {
            write!(f, "{s}")?;
        }
        Ok(())
    }
}

/// The kind of query
#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub enum QueryKind {
    /// A query against the root of a variant object, i.e., with `$`
    #[default]
    Root,
    /// A query against the current node within a variant object, i.e., with `@`
    Current,
}

impl Queryable for Query {
    fn query<'b, T: VariantValue>(&self, current: &'b T, root: &'b T) -> Vec<&'b T> {
        let mut result = match self.kind {
            QueryKind::Root => vec![root],
            QueryKind::Current => vec![current],
        };
        for segment in &self.segments {
            let mut r = Vec::new();
            for node in result {
                r.append(&mut segment.query(node, root));
            }
            result = r;
        }
        result
    }

    fn query_located<'b, T: VariantValue>(
        &self,
        current: &'b T,
        root: &'b T,
        parent: NormalizedPath<'b>,
    ) -> Vec<LocatedNode<'b, T>> {
        let mut result = match self.kind {
            QueryKind::Current => vec![LocatedNode::new(parent, current)],
            QueryKind::Root => vec![LocatedNode::new(Default::default(), root)],
        };
        for s in &self.segments {
            let mut r = vec![];
            for n in result {
                let loc = n.location();
                let node = n.node();
                r.append(&mut s.query_located(node, root, loc.clone()));
            }
            result = r;
        }
        result
    }
}
