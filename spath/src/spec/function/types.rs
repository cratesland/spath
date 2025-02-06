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

/// The type system of SPath values.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SPathType {
    /// A list of nodes.
    Nodes,
    /// A singular variant value.
    Value,
    /// A logical value.
    Logical,
}

impl fmt::Display for SPathType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SPathType::Nodes => write!(f, "nodes type"),
            SPathType::Logical => write!(f, "logical type"),
            SPathType::Value => write!(f, "value type"),
        }
    }
}

/// Function argument types.
///
/// This is used to describe the type of function argument to determine if it will be valid as a
/// parameter to the function it is being passed to.
///
/// The reason for having this type in addition to [`SPathType`] is that we need to have an
/// intermediate representation of arguments that are singular queries. This is because singular
/// queries can be used as an argument to both [`ValueType`] and [`NodesType`] parameters.
/// Therefore, we require a `Node` variant here to indicate that an argument may be converted into
/// either type of parameter.
#[doc(hidden)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FunctionArgType {
    /// Denotes a literal owned variant value
    Literal,
    /// Denotes a borrowed variant value from a singular query
    SingularQuery,
    /// Denotes a literal or borrowed variant value, used to represent functions that return
    /// [`ValueType`]
    Value,
    /// Denotes a node list, either from a filter query argument, or a function that returns
    /// [`NodesType`]
    NodeList,
    /// Denotes a logical, either from a logical expression, or from a function that returns
    /// [`LogicalType`]
    Logical,
}

impl fmt::Display for FunctionArgType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FunctionArgType::Literal => write!(f, "literal"),
            FunctionArgType::SingularQuery => write!(f, "singular query"),
            FunctionArgType::Value => write!(f, "value type"),
            FunctionArgType::NodeList => write!(f, "node list type"),
            FunctionArgType::Logical => write!(f, "logical type"),
        }
    }
}

impl FunctionArgType {
    pub fn converts_to(&self, spath_type: SPathType) -> bool {
        matches!(
            (self, spath_type),
            (
                FunctionArgType::Literal | FunctionArgType::Value,
                SPathType::Value
            ) | (
                FunctionArgType::SingularQuery,
                SPathType::Value | SPathType::Nodes | SPathType::Logical
            ) | (
                FunctionArgType::NodeList,
                SPathType::Nodes | SPathType::Logical
            ) | (FunctionArgType::Logical, SPathType::Logical),
        )
    }
}
