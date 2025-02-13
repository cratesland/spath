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

use std::ops::Deref;
use std::ops::DerefMut;

use crate::NodeList;
use crate::VariantValue;

/// SPath generic value.
#[derive(Debug)]
pub enum SPathValue<'a, T: VariantValue> {
    Nodes(NodeList<'a, T>),
    Logical(LogicalType),
    Node(&'a T),
    Value(T),
    Nothing,
}

impl<'a, T: VariantValue> SPathValue<'a, T> {
    /// Convert self to a node list if possible.
    pub fn into_nodes(self) -> Option<NodesType<'a, T>> {
        match self {
            SPathValue::Nodes(nodes) => Some(nodes.into()),
            _ => None,
        }
    }

    /// Convert self to a logical value if possible.
    ///
    /// §2.4.2. Type Conversion
    ///
    /// If the nodelist contains one or more nodes, the conversion result is LogicalTrue.
    ///
    /// If the nodelist is empty, the conversion result is LogicalFalse.
    pub fn into_logical(self) -> Option<LogicalType> {
        match self {
            SPathValue::Logical(logical) => Some(logical),
            SPathValue::Nodes(nodes) => Some(LogicalType::from(!nodes.is_empty())),
            _ => None,
        }
    }

    /// Convert self to a singular optional value if possible.
    pub fn into_value(self) -> Option<ValueType<'a, T>> {
        match self {
            SPathValue::Value(value) => Some(ValueType::Value(value)),
            SPathValue::Node(node) => Some(ValueType::Node(node)),
            SPathValue::Nothing => Some(ValueType::Nothing),
            _ => None,
        }
    }
}

/// SPath value representing a node list.
///
/// This is a thin wrapper around a [`NodeList`], and generally represents the result of an SPath
/// query. It may also be produced by a function.
#[derive(Debug, Default, PartialEq, Clone)]
pub struct NodesType<'a, T: VariantValue>(NodeList<'a, T>);

impl<'a, T: VariantValue> NodesType<'a, T> {
    /// Extract all inner nodes as a vector
    pub fn all(self) -> Vec<&'a T> {
        self.0.all()
    }
}

impl<'a, T: VariantValue> IntoIterator for NodesType<'a, T> {
    type Item = &'a T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, T: VariantValue> Deref for NodesType<'a, T> {
    type Target = NodeList<'a, T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: VariantValue> DerefMut for NodesType<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a, T: VariantValue> From<NodeList<'a, T>> for NodesType<'a, T> {
    fn from(value: NodeList<'a, T>) -> Self {
        Self(value)
    }
}

impl<'a, T: VariantValue> From<Vec<&'a T>> for NodesType<'a, T> {
    fn from(values: Vec<&'a T>) -> Self {
        Self(NodeList::new(values))
    }
}

/// SPath logical value.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum LogicalType {
    /// True
    True,
    /// False
    #[default]
    False,
}

impl From<LogicalType> for bool {
    fn from(value: LogicalType) -> Self {
        match value {
            LogicalType::True => true,
            LogicalType::False => false,
        }
    }
}

impl From<bool> for LogicalType {
    fn from(value: bool) -> Self {
        match value {
            true => Self::True,
            false => Self::False,
        }
    }
}

/// SPath value representing a singular value or Nothing.
#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub enum ValueType<'a, T: VariantValue> {
    /// This may come from a literal value declared in an SPath query, or be produced by a
    /// function.
    Value(T),
    /// This would be a reference to a location in the object being queried, i.e., the result
    /// of a singular query, or produced by a function.
    Node(&'a T),
    /// This would be the result of a singular query that does not result in any nodes, or be
    /// produced by a function.
    #[default]
    Nothing,
}

impl<T: VariantValue> ValueType<'_, T> {
    /// Convert to a reference of a variant value if possible.
    pub fn as_value(&self) -> Option<&T> {
        match self {
            ValueType::Value(v) => Some(v),
            ValueType::Node(v) => Some(*v),
            ValueType::Nothing => None,
        }
    }

    /// Check if this `ValueType` is nothing.
    pub fn is_nothing(&self) -> bool {
        matches!(self, ValueType::Nothing)
    }
}

impl<T: VariantValue> From<T> for ValueType<'_, T> {
    fn from(value: T) -> Self {
        Self::Value(value)
    }
}
