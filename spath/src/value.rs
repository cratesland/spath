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

//! Public structs and traits for variant (semi-structured) data values.

use std::fmt;

/// A literal variant value that can be represented in an SPath query.
#[derive(Debug, Clone)]
pub enum Literal {
    /// 64-bit integer.
    Int(i64),
    /// 64-bit floating point number.
    Float(f64),
    /// UTF-8 string.
    String(String),
    /// `true` or `false`.
    Bool(bool),
    /// `null`.
    Null,
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Int(n) => write!(f, "{n}"),
            Literal::Float(n) => write!(f, "{n:?}"),
            Literal::String(s) => write!(f, "'{s}'"),
            Literal::Bool(b) => write!(f, "{b}"),
            Literal::Null => write!(f, "null"),
        }
    }
}

/// A trait for converting a literal to a variant value.
pub trait FromLiteral {
    fn from_literal(literal: Literal) -> Option<Self>
    where
        Self: Sized;
}

/// A trait for any variant value.
pub trait VariantValue: FromLiteral {
    /// The type of the array variant.
    type VariantArray: ConcreteVariantArray<Value = Self>;
    /// The type of the object variant.
    type VariantObject: ConcreteVariantObject<Value = Self>;
    /// Whether the value is a null.
    fn is_null(&self) -> bool;
    /// Whether the value is a boolean.
    fn is_boolean(&self) -> bool;
    /// Whether the value is an array.
    fn is_array(&self) -> bool;
    /// Whether the value is an object.
    fn is_object(&self) -> bool;
    /// Convert the value to a bool; [`None`] if the value is not an array.
    fn as_bool(&self) -> Option<bool>;
    /// Convert the value to an array; [`None`] if the value is not an array.
    fn as_array(&self) -> Option<&Self::VariantArray>;
    /// Convert the value to an object; [`None`] if the value is not an object.
    fn as_object(&self) -> Option<&Self::VariantObject>;

    // §2.3.5.2.2 Comparisons
    /// Whether self is less than another value.
    fn is_less_than(&self, _other: &Self) -> bool {
        todo!()
    }
    /// Whether self is equal to another value.
    fn is_equal_to(&self, _other: &Self) -> bool {
        todo!()
    }
}

/// A trait for the concrete variant array type associated with a variant value.
pub trait ConcreteVariantArray {
    /// The type of the value in the array.
    type Value: VariantValue<VariantArray = Self>;
    /// Whether the array is empty.
    fn is_empty(&self) -> bool;
    /// The length of the array.
    fn len(&self) -> usize;
    /// Get the value at the given index; [`None`] if the index is out of bounds.
    fn get(&self, index: usize) -> Option<&Self::Value>;
    /// An iterator over the values in the array.
    fn iter(&self) -> impl Iterator<Item = &Self::Value>;
}

/// A trait for the concrete variant object type associated with a variant value.
pub trait ConcreteVariantObject {
    /// The type of the value in the object.
    type Value: VariantValue<VariantObject = Self>;
    /// Whether the object is empty.
    fn is_empty(&self) -> bool;
    /// The length of the object, i.e., the number of key-value pairs.
    fn len(&self) -> usize;
    /// Get the value for the given key; [`None`] if the key is not present.
    fn get(&self, key: &str) -> Option<&Self::Value>;
    /// Get the key-value pair for the given key; [`None`] if the key is not present.
    fn get_key_value(&self, key: &str) -> Option<(&String, &Self::Value)>;
    /// An iterator over the key-value pairs in the object.
    fn iter(&self) -> impl Iterator<Item = (&String, &Self::Value)>;
    /// An iterator over the values in the object.
    fn values(&self) -> impl Iterator<Item = &Self::Value>;
}
