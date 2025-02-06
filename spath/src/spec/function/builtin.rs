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

use num_traits::ToPrimitive;

use crate::spec::function::Function;
use crate::spec::function::FunctionArgType;
use crate::spec::function::SPathType;
use crate::spec::function::SPathValue;
use crate::spec::function::ValueType;
use crate::ConcreteVariantArray;
use crate::ConcreteVariantObject;
use crate::Literal;
use crate::VariantValue;

#[derive(Debug, Clone, Copy)]
pub struct LengthFn<T: VariantValue> {
    #[allow(dead_code)]
    t: std::marker::PhantomData<T>,
}

impl<T: VariantValue> Default for LengthFn<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: VariantValue> LengthFn<T> {
    pub fn new() -> Self {
        Self {
            t: std::marker::PhantomData,
        }
    }
}

impl<T: VariantValue> Function for LengthFn<T> {
    type Value = T;

    fn name(&self) -> &str {
        "length"
    }

    fn argument_types(&self) -> Vec<SPathType> {
        vec![SPathType::Value]
    }

    fn result_type(&self) -> FunctionArgType {
        FunctionArgType::Value
    }

    fn evaluate<'a>(
        &self,
        mut args: Vec<SPathValue<'a, Self::Value>>,
    ) -> SPathValue<'a, Self::Value> {
        assert_eq!(args.len(), 1);

        fn value_len<T: VariantValue>(t: &T) -> Option<usize> {
            if let Some(s) = t.as_str() {
                Some(s.chars().count())
            } else if let Some(a) = t.as_array() {
                Some(a.len())
            } else {
                t.as_object().map(|o| o.len())
            }
        }

        let value = args.pop().unwrap().into_value().unwrap();

        let len = match value {
            ValueType::Value(v) => value_len(&v),
            ValueType::Node(v) => value_len(v),
            ValueType::Nothing => None,
        }
        .and_then(|l| l.to_i64())
        .and_then(|len| T::from_literal(Literal::Int(len)));

        match len {
            Some(v) => SPathValue::Value(v),
            None => SPathValue::Nothing,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CountFn<T: VariantValue> {
    #[allow(dead_code)]
    t: std::marker::PhantomData<T>,
}

impl<T: VariantValue> Default for CountFn<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: VariantValue> CountFn<T> {
    pub fn new() -> Self {
        Self {
            t: std::marker::PhantomData,
        }
    }
}

impl<T: VariantValue> Function for CountFn<T> {
    type Value = T;

    fn name(&self) -> &str {
        "count"
    }

    fn argument_types(&self) -> Vec<SPathType> {
        vec![SPathType::Nodes]
    }

    fn result_type(&self) -> FunctionArgType {
        FunctionArgType::Value
    }

    fn evaluate<'a>(
        &self,
        mut args: Vec<SPathValue<'a, Self::Value>>,
    ) -> SPathValue<'a, Self::Value> {
        assert_eq!(args.len(), 1);

        let nodes = args.pop().unwrap().into_nodes().unwrap();

        let len = nodes
            .len()
            .to_i64()
            .and_then(|len| T::from_literal(Literal::Int(len)));

        match len {
            Some(v) => SPathValue::Value(v),
            None => SPathValue::Nothing,
        }
    }
}
