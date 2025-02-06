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
use crate::spec::query::Queryable;
use crate::spec::select_wildcard;
use crate::spec::selector::Selector;
use crate::ConcreteVariantArray;
use crate::ConcreteVariantObject;
use crate::LocatedNode;
use crate::NormalizedPath;
use crate::VariantValue;

/// A segment of a JSONPath query
#[derive(Debug, Clone)]
pub struct QuerySegment {
    /// The kind of segment
    pub kind: QuerySegmentKind,
    /// The segment
    pub segment: Segment,
}

impl QuerySegment {
    /// Is this a normal child segment
    pub fn is_child(&self) -> bool {
        matches!(self.kind, QuerySegmentKind::Child)
    }

    /// Is this a recursive descent child
    pub fn is_descendent(&self) -> bool {
        !self.is_child()
    }
}

impl fmt::Display for QuerySegment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if matches!(self.kind, QuerySegmentKind::Descendant) {
            write!(f, "..")?;
        }
        write!(f, "{}", self.segment)
    }
}

impl Queryable for QuerySegment {
    fn query<'b, T: VariantValue, R: FunctionRegistry<Value = T>>(
        &self,
        current: &'b T,
        root: &'b T,
        registry: &R,
    ) -> Vec<&'b T> {
        let mut query = self.segment.query(current, root, registry);
        if matches!(self.kind, QuerySegmentKind::Descendant) {
            query.append(&mut descend(self, current, root, registry));
        }
        query
    }

    fn query_located<'b, T: VariantValue, R: FunctionRegistry<Value = T>>(
        &self,
        current: &'b T,
        root: &'b T,
        registry: &R,
        parent: NormalizedPath<'b>,
    ) -> Vec<LocatedNode<'b, T>> {
        if matches!(self.kind, QuerySegmentKind::Descendant) {
            let mut result = self
                .segment
                .query_located(current, root, registry, parent.clone());
            result.append(&mut descend_paths(self, current, root, registry, parent));
            result
        } else {
            self.segment.query_located(current, root, registry, parent)
        }
    }
}

fn descend<'b, T: VariantValue, R: FunctionRegistry<Value = T>>(
    segment: &QuerySegment,
    current: &'b T,
    root: &'b T,
    registry: &R,
) -> Vec<&'b T> {
    let mut query = Vec::new();
    if let Some(list) = current.as_array() {
        for v in list.iter() {
            query.append(&mut segment.query(v, root, registry));
        }
    } else if let Some(obj) = current.as_object() {
        for v in obj.values() {
            query.append(&mut segment.query(v, root, registry));
        }
    }
    query
}

fn descend_paths<'b, T: VariantValue, R: FunctionRegistry<Value = T>>(
    segment: &QuerySegment,
    current: &'b T,
    root: &'b T,
    registry: &R,
    parent: NormalizedPath<'b>,
) -> Vec<LocatedNode<'b, T>> {
    let mut result = Vec::new();
    if let Some(list) = current.as_array() {
        for (i, v) in list.iter().enumerate() {
            result.append(&mut segment.query_located(v, root, registry, parent.clone_and_push(i)));
        }
    } else if let Some(obj) = current.as_object() {
        for (k, v) in obj.iter() {
            result.append(&mut segment.query_located(v, root, registry, parent.clone_and_push(k)));
        }
    }
    result
}

/// The kind of query segment
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum QuerySegmentKind {
    /// A normal child
    ///
    /// Addresses the direct descendant of the preceding segment
    Child,
    /// A descendant child
    ///
    /// Addresses all descendant children of the preceding segment, recursively
    Descendant,
}

/// Represents the different forms of SPath segment.
#[derive(Debug, Clone)]
pub enum Segment {
    /// Long hand segments contain multiple selectors inside square brackets.
    LongHand(Vec<Selector>),
    /// Dot-name selectors are a short form for representing keys in an object.
    DotName(String),
    /// The wildcard shorthand `.*`.
    Wildcard,
}

impl Segment {
    /// Does this segment extract a singular node.
    pub fn is_singular(&self) -> bool {
        match self {
            Segment::LongHand(selectors) => {
                if selectors.len() > 1 {
                    return false;
                }
                if let Some(s) = selectors.first() {
                    s.is_singular()
                } else {
                    // if the selector list is empty, this shouldn't be a valid
                    // JSONPath, but at least, it would be selecting nothing, and
                    // that could be considered singular, i.e., None.
                    true
                }
            }
            Segment::DotName(_) => true,
            Segment::Wildcard => false,
        }
    }

    /// Optionally produce self as a slice of selectors, from a long hand segment.
    pub fn as_long_hand(&self) -> Option<&[Selector]> {
        match self {
            Segment::LongHand(v) => Some(v.as_slice()),
            _ => None,
        }
    }

    /// Optionally produce self as a single name segment.
    pub fn as_dot_name(&self) -> Option<&str> {
        match self {
            Segment::DotName(s) => Some(s.as_str()),
            _ => None,
        }
    }
}

impl fmt::Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Segment::LongHand(selectors) => {
                write!(f, "[")?;
                for (i, s) in selectors.iter().enumerate() {
                    write!(
                        f,
                        "{s}{comma}",
                        comma = if i == selectors.len() - 1 { "" } else { "," }
                    )?;
                }
                write!(f, "]")?;
            }
            Segment::DotName(name) => write!(f, ".{name}")?,
            Segment::Wildcard => write!(f, ".*")?,
        }
        Ok(())
    }
}

impl Queryable for Segment {
    fn query<'b, T: VariantValue, R: FunctionRegistry<Value = T>>(
        &self,
        current: &'b T,
        root: &'b T,
        registry: &R,
    ) -> Vec<&'b T> {
        let mut result = Vec::new();
        match self {
            Segment::LongHand(selectors) => {
                for selector in selectors {
                    result.append(&mut selector.query(current, root, registry));
                }
            }
            Segment::DotName(key) => {
                if let Some(obj) = current.as_object() {
                    if let Some(v) = obj.get(key) {
                        result.push(v);
                    }
                }
            }
            Segment::Wildcard => select_wildcard(&mut result, current),
        }
        result
    }

    fn query_located<'b, T: VariantValue, R: FunctionRegistry<Value = T>>(
        &self,
        current: &'b T,
        root: &'b T,
        registry: &R,
        mut parent: NormalizedPath<'b>,
    ) -> Vec<LocatedNode<'b, T>> {
        let mut result = vec![];
        match self {
            Segment::LongHand(selectors) => {
                for s in selectors {
                    result.append(&mut s.query_located(current, root, registry, parent.clone()));
                }
            }
            Segment::DotName(name) => {
                if let Some((k, v)) = current.as_object().and_then(|o| o.get_key_value(name)) {
                    parent.push(k);
                    result.push(LocatedNode::new(parent, v));
                }
            }
            Segment::Wildcard => {
                if let Some(list) = current.as_array() {
                    for (i, v) in list.iter().enumerate() {
                        result.push(LocatedNode::new(parent.clone_and_push(i), v));
                    }
                } else if let Some(obj) = current.as_object() {
                    for (k, v) in obj.iter() {
                        result.push(LocatedNode::new(parent.clone_and_push(k), v));
                    }
                }
            }
        }
        result
    }
}
