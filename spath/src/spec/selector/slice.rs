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

//! Slice selectors for selecting array slices in SPath.

use crate::spec::integer::Integer;
use crate::spec::query::Queryable;
use crate::ConcreteVariantArray;
use crate::LocatedNode;
use crate::NormalizedPath;
use crate::VariantValue;

/// Array slice selector.
///
/// §2.3.4 Array Slice Selector.
///
/// Default Array Slice start and end Values:
///
/// | Condition | start     | end     |
/// |-----------|-----------|---------|
/// | step >= 0 | 0         | len     |
/// | step < 0  | len - 1   | -len - 1|
#[derive(Debug, PartialEq, Eq, Default, Clone, Copy)]
pub struct Slice {
    /// The start of the slice.
    ///
    /// This can be negative to start the slice from a position relative to the end of the array
    /// being sliced.
    start: Option<Integer>,
    /// The end of the slice.
    ///
    /// This can be negative to end the slice at a position relative to the end of the array being
    /// sliced.
    end: Option<Integer>,
    /// The step slice for the slice.
    ///
    /// This can be negative to step in reverse order.
    step: Option<Integer>,
}

impl std::fmt::Display for Slice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(start) = self.start {
            write!(f, "{start}")?;
        }
        write!(f, ":")?;
        if let Some(end) = self.end {
            write!(f, "{end}")?;
        }
        write!(f, ":")?;
        if let Some(step) = self.step {
            write!(f, "{step}")?;
        }
        Ok(())
    }
}

// these methods are either for internal use or test purposes
#[doc(hidden)]
impl Slice {
    /// Create a new slice selector.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the slice `start`.
    ///
    /// # Panics
    ///
    /// This will panic if the provided value is outside the range `[-(2^53) + 1, (2^53) - 1]`.
    pub fn with_start(mut self, start: i64) -> Self {
        self.start = Some(Integer::from_i64_unchecked(start));
        self
    }

    /// Set the slice `end`.
    ///
    /// # Panics
    ///
    /// This will panic if the provided value is outside the range `[-(2^53) + 1, (2^53) - 1]`.
    pub fn with_end(mut self, end: i64) -> Self {
        self.end = Some(Integer::from_i64_unchecked(end));
        self
    }

    /// Set the slice `step`.
    ///
    /// # Panics
    ///
    /// This will panic if the provided value is outside the range `[-(2^53) + 1, (2^53) - 1]`.
    pub fn with_step(mut self, step: i64) -> Self {
        self.step = Some(Integer::from_i64_unchecked(step));
        self
    }

    #[inline]
    fn bounds_on_forward_slice(&self, len: Integer) -> (Integer, Integer) {
        let start_default = self.start.unwrap_or(Integer::ZERO);
        let end_default = self.end.unwrap_or(len);
        let start = normalize_slice_index(start_default, len)
            .unwrap_or(Integer::ZERO)
            .max(Integer::ZERO);
        let end = normalize_slice_index(end_default, len)
            .unwrap_or(Integer::ZERO)
            .max(Integer::ZERO);
        let lower = start.min(len);
        let upper = end.min(len);
        (lower, upper)
    }

    #[inline]
    fn bounds_on_reverse_slice(&self, len: Integer) -> Option<(Integer, Integer)> {
        let start_default = self
            .start
            .or_else(|| len.checked_sub(Integer::from_i64_unchecked(1)))?;
        let end_default = self.end.or_else(|| {
            let l = len.checked_mul(Integer::from_i64_unchecked(-1))?;
            l.checked_sub(Integer::from_i64_unchecked(1))
        })?;
        let start = normalize_slice_index(start_default, len)
            .unwrap_or(Integer::ZERO)
            .max(Integer::from_i64_unchecked(-1));
        let end = normalize_slice_index(end_default, len)
            .unwrap_or(Integer::ZERO)
            .max(Integer::from_i64_unchecked(-1));
        let lower = end.min(
            len.checked_sub(Integer::from_i64_unchecked(1))
                .unwrap_or(len),
        );
        let upper = start.min(
            len.checked_sub(Integer::from_i64_unchecked(1))
                .unwrap_or(len),
        );
        Some((lower, upper))
    }
}

impl Queryable for Slice {
    fn query<'b, T: VariantValue>(&self, current: &'b T, _root: &'b T) -> Vec<&'b T> {
        if let Some(list) = current.as_array() {
            let mut result = Vec::new();
            let step = self.step.unwrap_or(Integer::from_i64_unchecked(1));
            if step == 0 {
                return vec![];
            }
            let Ok(len) = Integer::try_from(list.len()) else {
                return vec![];
            };
            if step > 0 {
                let (lower, upper) = self.bounds_on_forward_slice(len);
                let mut i = lower;
                while i < upper {
                    if let Some(v) = usize::try_from(i).ok().and_then(|i| list.get(i)) {
                        result.push(v);
                    }
                    i = if let Some(i) = i.checked_add(step) {
                        i
                    } else {
                        break;
                    };
                }
            } else {
                let Some((lower, upper)) = self.bounds_on_reverse_slice(len) else {
                    return vec![];
                };
                let mut i = upper;
                while lower < i {
                    if let Some(v) = usize::try_from(i).ok().and_then(|i| list.get(i)) {
                        result.push(v);
                    }
                    i = if let Some(i) = i.checked_add(step) {
                        i
                    } else {
                        break;
                    };
                }
            }
            result
        } else {
            vec![]
        }
    }

    fn query_located<'b, T: VariantValue>(
        &self,
        current: &'b T,
        _root: &'b T,
        parent: NormalizedPath<'b>,
    ) -> Vec<LocatedNode<'b, T>> {
        if let Some(list) = current.as_array() {
            let mut result = Vec::new();
            let step = self.step.unwrap_or(Integer::from_i64_unchecked(1));
            if step == 0 {
                return vec![];
            }
            let Ok(len) = Integer::try_from(list.len()) else {
                return vec![];
            };
            if step > 0 {
                let (lower, upper) = self.bounds_on_forward_slice(len);
                let mut i = lower;
                while i < upper {
                    if let Some((i, node)) = usize::try_from(i)
                        .ok()
                        .and_then(|i| list.get(i).map(|v| (i, v)))
                    {
                        result.push(LocatedNode::new(parent.clone_and_push(i), node));
                    }
                    i = if let Some(i) = i.checked_add(step) {
                        i
                    } else {
                        break;
                    };
                }
            } else {
                let Some((lower, upper)) = self.bounds_on_reverse_slice(len) else {
                    return vec![];
                };
                let mut i = upper;
                while lower < i {
                    if let Some((i, node)) = usize::try_from(i)
                        .ok()
                        .and_then(|i| list.get(i).map(|v| (i, v)))
                    {
                        result.push(LocatedNode::new(parent.clone_and_push(i), node));
                    }
                    i = if let Some(i) = i.checked_add(step) {
                        i
                    } else {
                        break;
                    };
                }
            }
            result
        } else {
            vec![]
        }
    }
}

fn normalize_slice_index(index: Integer, len: Integer) -> Option<Integer> {
    if index >= 0 {
        Some(index)
    } else {
        len.checked_sub(index.abs())
    }
}
