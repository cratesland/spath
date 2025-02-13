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
use std::marker::PhantomData;

use crate::spec::function::builtin::*;
use crate::VariantValue;

pub mod builtin;

mod expr;
pub use expr::*;

mod types;
pub use types::*;

mod value;
pub use value::*;

pub type Evaluator<T> = Box<dyn Fn(Vec<SPathValue<T>>) -> SPathValue<T>>;

pub struct Function<T: VariantValue> {
    name: &'static str,
    argument_types: Vec<SPathType>,
    result_type: SPathType,
    evaluator: Evaluator<T>,
}

impl<T: VariantValue> fmt::Debug for Function<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Function")
            .field("name", &self.name)
            .field("argument_types", &self.argument_types)
            .field("result_type", &self.result_type)
            .finish_non_exhaustive()
    }
}

impl<T: VariantValue> Function<T> {
    /// Create a new function instance.
    pub fn new(
        name: &'static str,
        argument_types: Vec<SPathType>,
        result_type: SPathType,
        evaluator: Evaluator<T>,
    ) -> Self {
        Self {
            name,
            argument_types,
            result_type,
            evaluator,
        }
    }

    /// The name of the function.
    pub fn name(&self) -> &str {
        self.name
    }

    /// The declared types of function's arguments.
    pub fn argument_types(&self) -> &[SPathType] {
        self.argument_types.as_slice()
    }

    /// The return type of the function.
    pub fn result_type(&self) -> SPathType {
        self.result_type
    }

    /// Evaluate the function with args.
    pub fn evaluate<'a>(&self, args: Vec<SPathValue<'a, T>>) -> SPathValue<'a, T> {
        (self.evaluator)(args)
    }

    /// Validate the type of function arguments.
    pub fn validate<Registry: FunctionRegistry<Value = T>>(
        &self,
        args: &[FunctionExprArg],
        registry: &Registry,
    ) -> Result<(), FunctionValidationError> {
        let argument_types = self.argument_types();

        if args.len() != argument_types.len() {
            return Err(FunctionValidationError::NumberOfArgsMismatch {
                name: self.name().to_string(),
                expected: 1,
                received: args.len(),
            });
        }

        for (i, arg) in args.iter().enumerate() {
            let ty = argument_types[i];
            let kind = arg.as_type_kind(registry)?;
            if !kind.converts_to(ty) {
                return Err(FunctionValidationError::MismatchTypeKind {
                    name: self.name().to_string(),
                    expected: ty,
                    received: kind,
                    position: i,
                });
            }
        }

        Ok(())
    }
}

#[doc(hidden)]
pub trait FunctionRegistry {
    type Value: VariantValue;
    fn get(&self, name: &str) -> Option<Function<Self::Value>>;
}

#[derive(Debug, Clone, Copy)]
pub struct BuiltinFunctionRegistry<T: VariantValue> {
    phantom: PhantomData<T>,
}

impl<T: VariantValue> Default for BuiltinFunctionRegistry<T> {
    fn default() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

impl<T: VariantValue> FunctionRegistry for BuiltinFunctionRegistry<T> {
    type Value = T;

    fn get(&self, name: &str) -> Option<Function<Self::Value>> {
        match name.to_lowercase().as_str() {
            "count" => Some(count()),
            "length" => Some(length()),
            "value" => Some(value()),
            #[cfg(feature = "regex")]
            "match" => Some(matches()),
            #[cfg(feature = "regex")]
            "search" => Some(search()),
            _ => None,
        }
    }
}
