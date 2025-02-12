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

use crate::spec::function::types::FunctionArgType;
use crate::spec::function::types::SPathType;
use crate::spec::function::value::LogicalType;
use crate::spec::function::value::SPathValue;
use crate::spec::function::FunctionRegistry;
use crate::spec::query::Query;
use crate::spec::query::Queryable;
use crate::spec::selector::filter::LogicalOrExpr;
use crate::spec::selector::filter::SingularQuery;
use crate::spec::selector::filter::TestFilter;
use crate::Literal;
use crate::NodeList;
use crate::VariantValue;

#[doc(hidden)]
#[derive(Debug, Clone)]
pub struct FunctionExpr {
    pub name: String,
    pub args: Vec<FunctionExprArg>,
    pub return_type: SPathType,
}

impl FunctionExpr {
    pub fn evaluate<'a, 'b: 'a, T: VariantValue, Registry: FunctionRegistry<Value = T>>(
        &'a self,
        current: &'b T,
        root: &'b T,
        registry: &Registry,
    ) -> SPathValue<'a, T> {
        let args: Vec<SPathValue<T>> = self
            .args
            .iter()
            .map(|a| a.evaluate(current, root, registry))
            .collect();
        // SAFETY: upon evaluation, the function is guaranteed to be validated
        let f = registry.get(self.name.as_str()).unwrap();
        f.evaluate(args)
    }

    pub fn validate<Registry: FunctionRegistry>(
        name: String,
        args: Vec<FunctionExprArg>,
        registry: &Registry,
    ) -> Result<(), FunctionValidationError> {
        let f = registry
            .get(name.as_str())
            .ok_or(FunctionValidationError::Undefined { name })?;
        f.validate(args.as_slice(), registry)
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
#[derive(Debug, Clone)]
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
    fn evaluate<'a, 'b: 'a, T: VariantValue, Registry: FunctionRegistry<Value = T>>(
        &'a self,
        current: &'b T,
        root: &'b T,
        registry: &Registry,
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
                let nodes = q.query(current, root, registry);
                SPathValue::Nodes(NodeList::new(nodes))
            }
            FunctionExprArg::LogicalExpr(l) => match l.test_filter(current, root, registry) {
                true => SPathValue::Logical(LogicalType::True),
                false => SPathValue::Logical(LogicalType::False),
            },
            FunctionExprArg::FunctionExpr(f) => f.evaluate(current, root, registry),
        }
    }

    pub fn as_type_kind<Registry: FunctionRegistry>(
        &self,
        registry: &Registry,
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
                .map(|f| f.result_type().as_function_arg_type())
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
    #[error("function {name} expects {expected} args, but received {received}")]
    NumberOfArgsMismatch {
        /// Function name.
        name: String,
        /// Expected number of arguments.
        expected: usize,
        /// Received number of arguments.
        received: usize,
    },
    /// The type of received argument does not match the function definition
    #[error("in function {name}, in argument position {position}, expected a type that converts to {expected}, received {received}"
    )]
    MismatchTypeKind {
        /// Function name.
        name: String,
        /// Expected type.
        expected: SPathType,
        /// Received type.
        received: FunctionArgType,
        /// Argument position.
        position: usize,
    },
    #[error("function with incorrect return type used")]
    IncorrectFunctionReturnType,
}
