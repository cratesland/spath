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

use crate::VariantValue;

pub mod builtin;

mod expr;
pub use expr::*;

mod types;
pub use types::*;

mod value;
pub use value::*;

#[doc(hidden)]
pub trait Function {
    type Value: VariantValue;
    /// The name of the function.
    fn name(&self) -> &str;
    /// The declared types of function's arguments.
    fn argument_types(&self) -> Vec<SPathType>;
    /// The return type of the function.
    fn result_type(&self) -> FunctionArgType;
    /// Evaluate the function with args.
    fn evaluate<'a>(&self, args: Vec<SPathValue<'a, Self::Value>>) -> SPathValue<'a, Self::Value>;

    /// Validate the type of function arguments.
    fn validate<R: FunctionRegistry<Value = Self::Value, Function = Self>>(
        &self,
        args: &[FunctionExprArg],
        registry: &R,
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
    type Function: Function<Value = Self::Value>;
    fn get(&self, name: &str) -> Option<Self::Function>;
}
