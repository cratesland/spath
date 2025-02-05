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

use std::cmp::Ordering;
use std::fmt;
use std::num::ParseIntError;
use std::num::TryFromIntError;
use std::str::FromStr;

/// An integer of [RFC 7493].
///
/// The value must be within the range `[-(2^53)+1, (2^53)-1]`.
///
/// [RFC 7493]: https://datatracker.ietf.org/doc/html/rfc7493#section-2.2
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Integer(i64);

/// The maximum allowed value, 2^53 - 1
const MAX: i64 = 9_007_199_254_740_992 - 1;
/// The minimum allowed value (-2^53) + 1
const MIN: i64 = -9_007_199_254_740_992 + 1;

#[inline]
fn check_i64_is_valid(v: i64) -> bool {
    (MIN..=MAX).contains(&v)
}

impl Integer {
    /// An [`Integer`] with the value 0
    pub const ZERO: Self = Self(0);

    fn try_new(value: i64) -> Result<Self, IntegerError> {
        if check_i64_is_valid(value) {
            Ok(Self(value))
        } else {
            Err(IntegerError::OutOfBounds)
        }
    }

    /// Get an [`Integer`] from an `i64`
    ///
    /// This is intended for initializing an integer with small, non-zero numbers.
    ///
    /// # Panics
    ///
    /// This will panic if the inputted value is out of the valid range
    /// `[-(2^53)+1, (2^53)-1]`.
    pub fn from_i64_unchecked(value: i64) -> Self {
        Self::try_new(value).expect("value is out of the valid range")
    }

    /// Take the absolute value, producing a new instance of [`Integer`]
    ///
    /// This is safe and will never panic since no instance of [`Integer`] can be constructed with
    /// a value that is outside the valid range and since the absolute of the minimum allowed value
    /// is the maximum value.
    pub fn abs(self) -> Self {
        Self(self.0.abs())
    }

    /// Add the two values, producing a new instance of [`Integer`] or `None` if the
    /// resulting value is outside the valid range `[-(2^53)+1, (2^53)-1]`
    pub fn checked_add(self, rhs: Self) -> Option<Self> {
        let i = self.0.checked_add(rhs.0)?;
        check_i64_is_valid(i).then_some(Self(i))
    }

    /// Subtract the `rhs` from `self`, producing a new instance of [`Integer`] or `None`
    /// if the resulting value is outside the valid range `[-(2^53)+1, (2^53)-1]`.
    pub fn checked_sub(self, rhs: Self) -> Option<Self> {
        let i = self.0.checked_sub(rhs.0)?;
        check_i64_is_valid(i).then_some(Self(i))
    }

    /// Multiply the two values, producing a new instance of [`Integer`] or `None` if the resulting
    /// value is outside the valid range `[-(2^53)+1, (2^53)-1]`.
    pub fn checked_mul(self, rhs: Self) -> Option<Self> {
        let i = self.0.checked_mul(rhs.0)?;
        check_i64_is_valid(i).then_some(Self(i))
    }
}

impl TryFrom<i64> for Integer {
    type Error = IntegerError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

macro_rules! impl_try_from {
    ($type:ty) => {
        impl TryFrom<$type> for Integer {
            type Error = IntegerError;

            fn try_from(value: $type) -> Result<Self, Self::Error> {
                i64::try_from(value)
                    .map_err(|_| IntegerError::OutOfBounds)
                    .and_then(Self::try_from)
            }
        }
    };
}

impl_try_from!(i128);
impl_try_from!(u64);
impl_try_from!(u128);
impl_try_from!(usize);
impl_try_from!(isize);

impl TryFrom<Integer> for usize {
    type Error = TryFromIntError;

    fn try_from(value: Integer) -> Result<Self, Self::Error> {
        Self::try_from(value.0)
    }
}

macro_rules! impl_from {
    ($type:ty) => {
        impl From<$type> for Integer {
            fn from(value: $type) -> Self {
                Self(value.into())
            }
        }
    };
}

impl_from!(i8);
impl_from!(i16);
impl_from!(i32);
impl_from!(u8);
impl_from!(u16);
impl_from!(u32);

impl FromStr for Integer {
    type Err = IntegerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<i64>().map_err(Into::into).and_then(Self::try_new)
    }
}

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq<i64> for Integer {
    fn eq(&self, other: &i64) -> bool {
        self.0.eq(other)
    }
}

impl PartialOrd<i64> for Integer {
    fn partial_cmp(&self, other: &i64) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

/// An error for the [`Integer`] type
#[derive(Debug, thiserror::Error)]
pub enum IntegerError {
    /// The provided value was outside the valid range `[-(2^53)+1, (2^53)-1]`.
    #[error("the provided integer was outside the valid range: [-(2^53)+1, (2^53)-1]")]
    OutOfBounds,
    /// Integer parsing error
    #[error(transparent)]
    Parse(#[from] ParseIntError),
}
