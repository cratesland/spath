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

use crate::value::ConcreteVariantArray;
use crate::value::ConcreteVariantObject;
use crate::value::VariantValue;
use crate::{ConcreteVariantOps, Literal};
use serde_json::Value;
use serde_json::{Map, Number};

impl VariantValue for Value {
    type VariantArray = Vec<Value>;
    type VariantObject = Map<String, Value>;
    type VariantOps = JsonValueOps;

    fn ops() -> Self::VariantOps {
        JsonValueOps
    }

    fn is_null(&self) -> bool {
        self.is_null()
    }

    fn is_boolean(&self) -> bool {
        self.is_boolean()
    }

    fn is_array(&self) -> bool {
        self.is_array()
    }

    fn is_object(&self) -> bool {
        self.is_object()
    }

    fn as_bool(&self) -> Option<bool> {
        self.as_bool()
    }

    fn as_array(&self) -> Option<&Self::VariantArray> {
        self.as_array()
    }

    fn as_object(&self) -> Option<&Self::VariantObject> {
        self.as_object()
    }
}

impl ConcreteVariantArray for Vec<Value> {
    type Value = Value;

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn len(&self) -> usize {
        (**self).len()
    }

    fn get(&self, index: usize) -> Option<&Self::Value> {
        (**self).get(index)
    }

    fn iter(&self) -> impl Iterator<Item = &Self::Value> {
        (**self).iter()
    }
}

impl ConcreteVariantObject for Map<String, Value> {
    type Value = Value;

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn get(&self, key: &str) -> Option<&Self::Value> {
        self.get(key)
    }

    fn get_key_value(&self, key: &str) -> Option<(&String, &Self::Value)> {
        self.get_key_value(key)
    }

    fn iter(&self) -> impl Iterator<Item = (&String, &Self::Value)> {
        self.iter()
    }

    fn values(&self) -> impl Iterator<Item = &Self::Value> {
        self.values()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct JsonValueOps;

impl ConcreteVariantOps for JsonValueOps {
    type Value = Value;

    fn from_literal(literal: Literal) -> Option<Self::Value> {
        match literal {
            Literal::Int(v) => Some(Value::Number(v.into())),
            Literal::UInt(v) => Some(Value::Number(v.into())),
            Literal::Float(v) => Number::from_f64(v).map(Value::Number),
            Literal::String(v) => Some(Value::String(v)),
            Literal::Bool(b) => Some(Value::Bool(b)),
            Literal::Null => Some(Value::Null),
        }
    }
}
