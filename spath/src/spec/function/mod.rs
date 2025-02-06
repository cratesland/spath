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

use crate::VariantValue;

mod expr;
pub use expr::*;

mod types;
pub use types::*;

mod value;
pub use value::*;

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
