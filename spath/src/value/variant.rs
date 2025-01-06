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

use crate::Map;
use crate::Number;

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
    Array(Vec<Value>),
    Object(Map),
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
