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

use std::collections::VecDeque;
use std::fmt;
use std::ops::Deref;
use std::ops::DerefMut;

use crate::spec::query::Query;
use crate::spec::query::Queryable;
use crate::spec::selector::filter::LogicalOrExpr;
use crate::spec::selector::filter::SingularQuery;
use crate::spec::selector::filter::TestFilter;
use crate::Literal;
use crate::NodeList;
use crate::VariantValue;

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

/// SPath value representing a node list.
///
/// This is a thin wrapper around a [`NodeList`], and generally represents the result of an SPath
/// query. It may also be produced by a function.
#[derive(Debug, Default, PartialEq, Clone)]
pub struct NodesType<'a, T: VariantValue>(NodeList<'a, T>);

impl<'a, T: VariantValue> NodesType<'a, T> {
    #[doc(hidden)]
    pub const fn spath_type() -> SPathType {
        SPathType::Nodes
    }

    #[doc(hidden)]
    pub const fn function_type() -> FunctionArgType {
        FunctionArgType::NodeList
    }

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

impl LogicalType {
    /// Returns the spath type.
    pub const fn spath_type() -> SPathType {
        SPathType::Logical
    }

    #[doc(hidden)]
    pub const fn function_type() -> FunctionArgType {
        FunctionArgType::Logical
    }
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
    #[doc(hidden)]
    pub const fn spath_type() -> SPathType {
        SPathType::Value
    }

    #[doc(hidden)]
    pub const fn function_type() -> FunctionArgType {
        FunctionArgType::Value
    }

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

/// SPath generic value.
#[derive(Debug)]
pub enum SPathValue<'a, T: VariantValue> {
    Nodes(NodeList<'a, T>),
    Logical(LogicalType),
    Node(&'a T),
    Value(T),
    Nothing,
}

#[doc(hidden)]
pub trait Function {
    type Value: VariantValue;
    fn name(&self) -> &str;
    fn result_type(&self) -> FunctionArgType;
    fn validate(&self, args: &[FunctionExprArg]) -> Result<(), FunctionValidationError>;
    fn evaluate<'a>(
        &self,
        args: VecDeque<SPathValue<'a, Self::Value>>,
    ) -> SPathValue<'a, Self::Value>;
}

#[doc(hidden)]
pub trait FunctionRegistry {
    type Value: VariantValue;
    type Function: Function<Value = Self::Value>;
    fn get(&self, name: &str) -> Option<Self::Function>;
}

#[doc(hidden)]
#[derive(Debug)]
pub struct FunctionExpr {
    pub name: String,
    pub args: Vec<FunctionExprArg>,
    pub return_type: FunctionArgType,
}

impl FunctionExpr {
    pub fn evaluate<'a, 'b: 'a, T: VariantValue, R: FunctionRegistry<Value = T>>(
        &'a self,
        current: &'b T,
        root: &'b T,
        registry: &R,
    ) -> SPathValue<'a, T> {
        let args: VecDeque<SPathValue<T>> = self
            .args
            .iter()
            .map(|a| a.evaluate(current, root, registry))
            .collect();
        // SAFETY: upon evaluation, the function is guaranteed to be validated
        let f = registry.get(self.name.as_str()).unwrap();
        f.evaluate(args)
    }

    pub fn validate<R: FunctionRegistry>(
        name: String,
        args: Vec<FunctionExprArg>,
        registry: &R,
    ) -> Result<(), FunctionValidationError> {
        let f = registry
            .get(name.as_str())
            .ok_or(FunctionValidationError::Undefined { name })?;
        f.validate(args.as_slice())
    }
}

impl fmt::Display for FunctionExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{name}(", name = self.name)?;
        for (i, arg) in self.args.iter().enumerate() {
            write!(
                f,
                "{arg}{comma}",
                comma = if i == self.args.len() - 1 { "" } else { "," }
            )?;
        }
        write!(f, ")")
    }
}

#[doc(hidden)]
#[derive(Debug)]
pub enum FunctionExprArg {
    Literal(Literal),
    SingularQuery(SingularQuery),
    FilterQuery(Query),
    LogicalExpr(LogicalOrExpr),
    FunctionExpr(FunctionExpr),
}

impl fmt::Display for FunctionExprArg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FunctionExprArg::Literal(lit) => write!(f, "{lit}"),
            FunctionExprArg::FilterQuery(query) => write!(f, "{query}"),
            FunctionExprArg::SingularQuery(sq) => write!(f, "{sq}"),
            FunctionExprArg::LogicalExpr(log) => write!(f, "{log}"),
            FunctionExprArg::FunctionExpr(func) => write!(f, "{func}"),
        }
    }
}

impl FunctionExprArg {
    fn evaluate<'a, 'b: 'a, T: VariantValue, R: FunctionRegistry<Value = T>>(
        &'a self,
        current: &'b T,
        root: &'b T,
        registry: &R,
    ) -> SPathValue<'a, T> {
        match self {
            FunctionExprArg::Literal(lit) => match T::from_literal(lit.clone()) {
                None => SPathValue::Nothing,
                Some(v) => SPathValue::Value(v),
            },
            FunctionExprArg::SingularQuery(q) => match q.eval_query(current, root) {
                Some(n) => SPathValue::Node(n),
                None => SPathValue::Nothing,
            },
            FunctionExprArg::FilterQuery(q) => {
                let nodes = q.query(current, root);
                SPathValue::Nodes(NodeList::new(nodes))
            }
            FunctionExprArg::LogicalExpr(l) => match l.test_filter(current, root) {
                true => SPathValue::Logical(LogicalType::True),
                false => SPathValue::Logical(LogicalType::False),
            },
            FunctionExprArg::FunctionExpr(f) => f.evaluate(current, root, registry),
        }
    }

    pub fn as_type_kind<R: FunctionRegistry>(
        &self,
        registry: &R,
    ) -> Result<FunctionArgType, FunctionValidationError> {
        match self {
            FunctionExprArg::Literal(_) => Ok(FunctionArgType::Literal),
            FunctionExprArg::SingularQuery(_) => Ok(FunctionArgType::SingularQuery),
            FunctionExprArg::FilterQuery(query) => {
                if query.is_singular() {
                    Ok(FunctionArgType::SingularQuery)
                } else {
                    Ok(FunctionArgType::NodeList)
                }
            }
            FunctionExprArg::LogicalExpr(_) => Ok(FunctionArgType::Logical),
            FunctionExprArg::FunctionExpr(func) => registry
                .get(func.name.as_str())
                .map(|f| f.result_type())
                .ok_or_else(|| FunctionValidationError::Undefined {
                    name: func.name.to_string(),
                }),
        }
    }
}

/// An error occurred while validating a function
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum FunctionValidationError {
    /// Function not defined in inventory
    #[error("function name '{name}' is not defined")]
    Undefined {
        /// The name of the function
        name: String,
    },
    /// Mismatch in number of function arguments
    #[error("expected {expected} args, but received {received}")]
    NumberOfArgsMismatch {
        /// Expected number of arguments
        expected: usize,
        /// Received number of arguments
        received: usize,
    },
    /// The type of received argument does not match the function definition
    #[error("in function {name}, in argument position {position}, expected a type that converts to {expected}, received {received}")]
    MismatchTypeKind {
        /// Function name
        name: String,
        /// Expected type
        expected: SPathType,
        /// Received type
        received: FunctionArgType,
        /// Argument position
        position: usize,
    },
    #[error("function with incorrect return type used")]
    IncorrectFunctionReturnType,
}
