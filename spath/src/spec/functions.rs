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
use std::sync::Arc;

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
    Nodelist,
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
            FunctionArgType::Nodelist => write!(f, "nodes type"),
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
                FunctionArgType::Nodelist,
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
        FunctionArgType::Nodelist
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
pub type Validator<T> =
    Arc<dyn Fn(&[FunctionExprArg<T>]) -> Result<(), FunctionValidationError> + Send + Sync>;

#[doc(hidden)]
pub type Evaluator<T> =
    Arc<dyn for<'a> Fn(VecDeque<SPathValue<'a, T>>) -> SPathValue<'a, T> + Sync + Send>;

#[doc(hidden)]
pub struct Function<T: VariantValue> {
    pub name: &'static str,
    pub result_type: FunctionArgType,
    pub validator: Validator<T>,
    pub evaluator: Evaluator<T>,
}

impl<T: VariantValue> fmt::Debug for Function<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Function")
            .field("name", &self.name)
            .field("result_type", &self.result_type)
            .finish()
    }
}

impl<T: VariantValue> Function<T> {
    pub const fn new(
        name: &'static str,
        result_type: FunctionArgType,
        evaluator: Evaluator<T>,
        validator: Validator<T>,
    ) -> Self {
        Self {
            name,
            result_type,
            evaluator,
            validator,
        }
    }
}

#[doc(hidden)]
#[derive(Debug)]
pub struct FunctionExpr<V, T: VariantValue> {
    pub name: String,
    pub args: Vec<FunctionExprArg<T>>,
    pub return_type: FunctionArgType,
    pub validated: V,
}

impl<T: VariantValue> FunctionExpr<Validated<T>, T> {
    pub fn evaluate<'a, 'b: 'a>(&'a self, current: &'b T, root: &'b T) -> SPathValue<'a, T> {
        let args: VecDeque<SPathValue<T>> = self
            .args
            .iter()
            .map(|a| a.evaluate(current, root))
            .collect();
        (self.validated.evaluator)(args)
    }
}

impl<T: VariantValue> FunctionExpr<NotValidated, T> {
    pub fn validate(
        name: String,
        args: Vec<FunctionExprArg<T>>,
        registry: FunctionRegistry<T>,
    ) -> Result<FunctionExpr<Validated<T>, T>, FunctionValidationError> {
        for f in registry.functions.iter() {
            if f.name == name {
                (f.validator)(args.as_slice())?;
                return Ok(FunctionExpr {
                    name,
                    args,
                    return_type: f.result_type,
                    validated: Validated {
                        evaluator: f.evaluator.clone(),
                    },
                });
            }
        }
        Err(FunctionValidationError::Undefined { name })
    }
}

impl<V, T: VariantValue> fmt::Display for FunctionExpr<V, T> {
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
pub struct NotValidated;

#[doc(hidden)]
pub struct Validated<T: VariantValue> {
    pub evaluator: Evaluator<T>,
}

impl<T: VariantValue> fmt::Debug for Validated<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Validated").finish()
    }
}

#[doc(hidden)]
#[derive(Debug)]
pub struct FunctionRegistry<T: VariantValue> {
    functions: Vec<Function<T>>,
}

#[doc(hidden)]
#[derive(Debug)]
pub enum FunctionExprArg<T: VariantValue> {
    Literal(Literal),
    SingularQuery(SingularQuery),
    FilterQuery(Query),
    LogicalExpr(LogicalOrExpr),
    FunctionExpr(FunctionExpr<Validated<T>, T>),
}

impl<T: VariantValue> fmt::Display for FunctionExprArg<T> {
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

impl<T: VariantValue> FunctionExprArg<T> {
    fn evaluate<'a, 'b: 'a>(&'a self, current: &'b T, root: &'b T) -> SPathValue<'a, T> {
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
            FunctionExprArg::FunctionExpr(f) => f.evaluate(current, root),
        }
    }

    pub fn as_type_kind(
        &self,
        registry: FunctionRegistry<T>,
    ) -> Result<FunctionArgType, FunctionValidationError> {
        match self {
            FunctionExprArg::Literal(_) => Ok(FunctionArgType::Literal),
            FunctionExprArg::SingularQuery(_) => Ok(FunctionArgType::SingularQuery),
            FunctionExprArg::FilterQuery(query) => {
                if query.is_singular() {
                    Ok(FunctionArgType::SingularQuery)
                } else {
                    Ok(FunctionArgType::Nodelist)
                }
            }
            FunctionExprArg::LogicalExpr(_) => Ok(FunctionArgType::Logical),
            FunctionExprArg::FunctionExpr(func) => {
                for f in registry.functions.iter() {
                    if f.name == func.name.as_str() {
                        return Ok(f.result_type);
                    }
                }
                Err(FunctionValidationError::Undefined {
                    name: func.name.to_string(),
                })
            }
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
