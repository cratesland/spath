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

use std::cmp::max;
use std::cmp::min;
use std::cmp::Ordering;

use num_traits::ToPrimitive;

use crate::spec::functions::FunctionRegistry;
use crate::spec::query::Queryable;
use crate::ConcreteVariantArray;
use crate::LocatedNode;
use crate::NormalizedPath;
use crate::VariantValue;

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
    /// The start of the slice, inclusive.
    ///
    /// This can be negative to start the slice from a position relative to the end of the array
    /// being sliced.
    start: Option<i64>,
    /// The end of the slice, exclusive.
    ///
    /// This can be negative to end the slice at a position relative to the end of the array being
    /// sliced.
    end: Option<i64>,
    /// The step slice for the slice. Default to 1.
    ///
    /// This can be negative to step in reverse order.
    step: Option<i64>,
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

// §2.3.4.2.2. (Array Slice Selector) Normative Semantics
fn bounds(start: i64, end: i64, step: i64, len: i64) -> (i64, i64) {
    fn normalize(i: i64, len: i64) -> i64 {
        if i < 0 {
            len + i
        } else {
            i
        }
    }

    let start = normalize(start, len);
    let end = normalize(end, len);

    if step >= 0 {
        let lower = min(max(start, 0), len);
        let upper = min(max(end, 0), len);
        (lower, upper)
    } else {
        let upper = min(max(start, -1), len - 1);
        let lower = min(max(end, -1), len - 1);
        (lower, upper)
    }
}

impl Slice {
    /// Create a new slice selector.
    pub fn new(start: Option<i64>, end: Option<i64>, step: Option<i64>) -> Self {
        Self { start, end, step }
    }

    fn select<'b, T, N, F>(&self, current: &'b T, make_node: F) -> Vec<N>
    where
        T: VariantValue,
        N: 'b,
        F: Fn(usize, &'b T) -> N,
    {
        let vec = match current.as_array() {
            Some(vec) => vec,
            None => return vec![],
        };

        let (start, end, step) = (self.start, self.end, self.step.unwrap_or(1));
        if step == 0 {
            // §2.3.4.2.2. Normative Semantics
            // When step = 0, no elements are selected, and the result array is empty.
            return vec![];
        }

        let len = vec.len().to_i64().unwrap_or(i64::MAX);
        let (start, end) = if step >= 0 {
            match (start, end) {
                (Some(start), Some(end)) => (start, end),
                (Some(start), None) => (start, len),
                (None, Some(end)) => (0, end),
                (None, None) => (0, len),
            }
        } else {
            match (start, end) {
                (Some(start), Some(end)) => (start, end),
                (Some(start), None) => (start, -len - 1),
                (None, Some(end)) => (len - 1, end),
                (None, None) => (len - 1, -len - 1),
            }
        };

        let (lower, upper) = bounds(start, end, step, len);
        let mut selected = vec![];
        match step.cmp(&0) {
            Ordering::Greater => {
                // step > 0
                let mut i = lower;
                while i < upper {
                    let node = vec.get(i as usize).unwrap();
                    selected.push(make_node(i as usize, node));
                    i += step;
                }
            }
            Ordering::Less => {
                // step < 0
                let mut i = upper;
                while lower < i {
                    let node = vec.get(i as usize).unwrap();
                    selected.push(make_node(i as usize, node));
                    i += step;
                }
            }
            Ordering::Equal => unreachable!("step is guaranteed not zero here"),
        }
        selected
    }
}

impl Queryable for Slice {
    fn query<'b, T: VariantValue, R: FunctionRegistry<Value = T>>(
        &self,
        current: &'b T,
        _root: &'b T,
        _registry: &R,
    ) -> Vec<&'b T> {
        self.select(current, |_, node| node)
    }

    fn query_located<'b, T: VariantValue, R: FunctionRegistry<Value = T>>(
        &self,
        current: &'b T,
        _root: &'b T,
        _registry: &R,
        parent: NormalizedPath<'b>,
    ) -> Vec<LocatedNode<'b, T>> {
        self.select(current, |i, node| {
            LocatedNode::new(parent.clone_and_push(i), node)
        })
    }
}
