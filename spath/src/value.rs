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

use std::collections::BTreeMap;
use std::fmt;
use std::hash::Hash;
use std::hash::Hasher;

use num_cmp::NumCmp;

/// An ordered floating point number. Provided by the [`ordered-float`] crate.
pub type F64 = ordered_float::OrderedFloat<f64>;

/// The type of array of values.
pub type Array = Vec<Value>;

/// The type of object values.
pub type Object = BTreeMap<String, Value>;

/// A variant value.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Value {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Timestamp(jiff::Timestamp),
    Interval(jiff::SignedDuration),
    Binary(Vec<u8>),
    Array(Array),
    Object(Object),
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Null => write!(f, "null"),
            Value::Bool(b) => write!(f, "{b}"),
            Value::Number(n) => write!(f, "{n:?}"),
            Value::String(s) => write!(f, "'{s}'"),
            Value::Timestamp(t) => write!(f, "{t:.6}"),
            Value::Interval(i) => write!(f, "{i}"),
            Value::Binary(b) => write!(f, "{b:?}"),
            Value::Array(a) => {
                write!(f, "[")?;
                for (i, v) in a.iter().enumerate() {
                    if i > 0 {
                        write!(f, ",")?;
                    }
                    write!(f, "{:?}", v)?;
                }
                write!(f, "]")?;
                Ok(())
            }
            Value::Object(o) => {
                write!(f, "{{")?;
                for (i, (k, v)) in o.iter().enumerate() {
                    if i > 0 {
                        write!(f, ",")?;
                    }
                    write!(f, "{:?}:{:?}", k, v)?;
                }
                write!(f, "}}")?;
                Ok(())
            }
        }
    }
}

/// A variant number value.
#[derive(Copy, Clone)]
pub enum Number {
    I64(i64),
    U64(u64),
    F64(F64),
}

impl From<i64> for Number {
    fn from(n: i64) -> Self {
        Number::I64(n)
    }
}

impl From<u64> for Number {
    fn from(n: u64) -> Self {
        Number::U64(n)
    }
}

impl From<f64> for Number {
    fn from(n: f64) -> Self {
        Number::F64(F64::from(n))
    }
}

impl fmt::Debug for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Number::I64(n) => write!(f, "{n:?}"),
            Number::U64(n) => write!(f, "{n:?}"),
            Number::F64(n) => write!(f, "{n:?}"),
        }
    }
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == std::cmp::Ordering::Equal
    }
}

impl Eq for Number {}

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Number {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Number::I64(a), Number::I64(b)) => a.cmp(b),
            (Number::U64(a), Number::U64(b)) => a.cmp(b),
            (Number::F64(a), Number::F64(b)) => a.cmp(b),

            (Number::I64(a), Number::U64(b)) => NumCmp::num_cmp(*a, *b).unwrap(),
            (Number::U64(a), Number::I64(b)) => NumCmp::num_cmp(*a, *b).unwrap(),
            (Number::F64(a), Number::I64(b)) => NumCmp::num_cmp(*a, *b).unwrap(),
            (Number::F64(a), Number::U64(b)) => NumCmp::num_cmp(*a, *b).unwrap(),

            (Number::I64(a), Number::F64(b)) => NumCmp::num_cmp(*b, *a).unwrap().reverse(),
            (Number::U64(a), Number::F64(b)) => NumCmp::num_cmp(*b, *a).unwrap().reverse(),
        }
    }
}

impl Hash for Number {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Number::I64(n) => n.hash(state),
            Number::U64(n) => n.hash(state),
            Number::F64(n) => n.hash(state),
        }
    }
}
