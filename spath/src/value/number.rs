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
use std::hash::Hash;
use std::hash::Hasher;

use num_cmp::NumCmp;
use ordered_float::OrderedFloat;

/// A variant number value.
#[derive(Copy, Clone)]
pub enum Number {
    I64(i64),
    U64(u64),
    F64(OrderedFloat<f64>),
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
        Number::F64(n.into())
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
