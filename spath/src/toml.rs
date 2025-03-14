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

use num_cmp::NumCmp;
use toml::Table;
use toml::Value;

use crate::spec::function;
use crate::value::ConcreteVariantArray;
use crate::value::ConcreteVariantObject;
use crate::value::VariantValue;
use crate::FromLiteral;
use crate::Literal;

pub type BuiltinFunctionRegistry = function::BuiltinFunctionRegistry<Value>;

impl FromLiteral for Value {
    fn from_literal(literal: Literal) -> Option<Self> {
        match literal {
            Literal::Int(v) => Some(Value::Integer(v)),
            Literal::Float(v) => Some(Value::Float(v)),
            Literal::String(v) => Some(Value::String(v)),
            Literal::Bool(v) => Some(Value::Boolean(v)),
            Literal::Null => None,
        }
    }
}

impl VariantValue for Value {
    type VariantArray = Vec<Value>;
    type VariantObject = Table;

    fn is_null(&self) -> bool {
        // toml 1.0 does not have null
        //
        // @see https://github.com/toml-lang/toml/issues/975
        // @see https://github.com/toml-lang/toml.io/issues/70
        // @see https://github.com/toml-lang/toml/issues/30
        false
    }

    fn is_boolean(&self) -> bool {
        self.is_bool()
    }

    fn is_string(&self) -> bool {
        self.is_str()
    }

    fn is_array(&self) -> bool {
        self.is_array()
    }

    fn is_object(&self) -> bool {
        self.is_table()
    }

    fn as_bool(&self) -> Option<bool> {
        self.as_bool()
    }

    fn as_str(&self) -> Option<&str> {
        self.as_str()
    }

    fn as_array(&self) -> Option<&Self::VariantArray> {
        self.as_array()
    }

    fn as_object(&self) -> Option<&Self::VariantObject> {
        self.as_table()
    }

    fn is_less_than(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Integer(l), Value::Float(r)) => NumCmp::num_lt(*l, *r),
            (Value::Float(l), Value::Integer(r)) => NumCmp::num_lt(*l, *r),
            (Value::String(l), Value::String(r)) => l < r,
            _ => false,
        }
    }

    fn is_equal_to(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Integer(l), Value::Float(r)) => NumCmp::num_eq(*l, *r),
            (Value::Float(l), Value::Integer(r)) => NumCmp::num_eq(*l, *r),
            _ => self == other,
        }
    }
}

impl ConcreteVariantArray for Vec<Value> {
    type Value = Value;

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn get(&self, index: usize) -> Option<&Self::Value> {
        (**self).get(index)
    }

    fn iter(&self) -> impl Iterator<Item = &Self::Value> {
        (**self).iter()
    }
}

impl ConcreteVariantObject for Table {
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
