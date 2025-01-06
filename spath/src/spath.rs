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

use std::cmp::max;
use std::cmp::min;
use std::cmp::Ordering;

use num_traits::ToPrimitive;

use crate::parser::ast::Segment;
use crate::parser::ast::Selector;
use crate::parser::runner::run_parser;
use crate::BindError;
use crate::Value;

#[derive(Debug, Clone)]
pub struct SPath {
    segments: Vec<EvalSegment>,
}

impl SPath {
    pub fn new(source: &str) -> Result<Self, BindError> {
        let segments = run_parser(source).map_err(|err| BindError(format!("{err}")))?;
        let binder = Binder {};
        Ok(binder.bind(segments))
    }

    pub fn eval(&self, root: &Value) -> Option<Value> {
        let mut result = root.clone();
        for segment in &self.segments {
            if let Some(res) = segment.eval(root, result) {
                result = res;
            } else {
                return None;
            }
        }
        Some(result.clone())
    }
}

#[derive(Debug, Clone)]
enum EvalSegment {
    /// §2.5.1 Child Segment.
    Child {
        /// The selectors of the child segment.
        selectors: Vec<EvalSelector>,
    },
    /// §2.5.2 Descendant Segment.
    Descendant {
        /// The selectors of the descendant segment.
        selectors: Vec<EvalSelector>,
    },
}

impl EvalSegment {
    fn eval(&self, root: &Value, value: Value) -> Option<Value> {
        match self {
            EvalSegment::Child { selectors } => {
                let mut result = vec![];
                for selector in selectors {
                    if let Some(res) = selector.eval(root, &value) {
                        result.push(res);
                    }
                }

                if selectors.len() <= 1 {
                    result.pop()
                } else {
                    Some(Value::Array(result))
                }
            }
            EvalSegment::Descendant {
                selectors: _selectors,
            } => {
                todo!("descendant segment")
            }
        }
    }
}

#[derive(Debug, Clone)]
enum EvalSelector {
    /// §2.3.2 Wildcard Selector.
    Wildcard,
    /// §2.3.1 Name Selector.
    Identifier {
        /// The name of the selector.
        name: String,
    },
    /// §2.3.3 Index Selector.
    Index {
        /// The index of the selector.
        index: i64,
    },
    /// §2.3.4 Array Slice Selector.
    ///
    /// Default Array Slice start and end Values:
    ///
    /// | Condition | start     | end     |
    /// |-----------|-----------|---------|
    /// | step >= 0 | 0         | len     |
    /// | step < 0  | len - 1   | -len - 1|
    Slice {
        /// The start index of the slice, inclusive.
        start: Option<i64>,
        /// The end index of the slice, exclusive.
        end: Option<i64>,
        /// The step to iterate the slice. Default to 1.
        step: i64,
    },
}

impl EvalSelector {
    fn eval(&self, _root: &Value, value: &Value) -> Option<Value> {
        match self {
            EvalSelector::Wildcard => Some(value.clone()),
            EvalSelector::Identifier { name } => {
                if let Value::Object(map) = value {
                    map.get(name).cloned()
                } else {
                    None
                }
            }
            EvalSelector::Index { index } => {
                if let Value::Array(vec) = value {
                    let index = resolve_index(*index, vec.len())?;
                    vec.get(index).cloned()
                } else {
                    None
                }
            }
            EvalSelector::Slice { start, end, step } => {
                if let Value::Array(vec) = value {
                    let step = *step;
                    if step == 0 {
                        // §2.3.4.2.2. Normative Semantics
                        // When step = 0, no elements are selected, and the result array is empty.
                        return Some(Value::Array(vec![]));
                    }

                    let len = vec.len().to_i64().unwrap_or(i64::MAX);
                    let (start, end) = if step >= 0 {
                        match (start, end) {
                            (Some(start), Some(end)) => (*start, *end),
                            (Some(start), None) => (*start, len),
                            (None, Some(end)) => (0, *end),
                            (None, None) => (0, len),
                        }
                    } else {
                        match (start, end) {
                            (Some(start), Some(end)) => (*start, *end),
                            (Some(start), None) => (*start, -len - 1),
                            (None, Some(end)) => (len - 1, *end),
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
                                selected.push(vec[i as usize].clone());
                                i += step;
                            }
                        }
                        Ordering::Less => {
                            // step < 0
                            let mut i = upper;
                            while lower < i {
                                selected.push(vec[i as usize].clone());
                                i += step;
                            }
                        }
                        Ordering::Equal => unreachable!("step is guaranteed not zero here"),
                    }
                    Some(Value::Array(selected))
                } else {
                    None
                }
            }
        }
    }
}

// §2.3.3.2. (Index Selector) Semantics
fn resolve_index(index: i64, len: usize) -> Option<usize> {
    let index = if index >= 0 {
        index.to_usize()?
    } else {
        let index = len.to_i64().unwrap_or(i64::MAX) + index;
        index.to_usize()?
    };

    if index < len {
        Some(index)
    } else {
        None
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

#[derive(Default, Debug, Clone)]
#[non_exhaustive]
pub struct Binder {}

impl Binder {
    fn bind(&self, segments: Vec<Segment>) -> SPath {
        let segments = segments.into_iter().map(|s| self.bind_segment(s)).collect();
        SPath { segments }
    }

    fn bind_segment(&self, segment: Segment) -> EvalSegment {
        match segment {
            Segment::Child { selectors } => EvalSegment::Child {
                selectors: selectors
                    .into_iter()
                    .map(|s| self.bind_selector(s))
                    .collect(),
            },
            Segment::Descendant { selectors } => EvalSegment::Descendant {
                selectors: selectors
                    .into_iter()
                    .map(|s| self.bind_selector(s))
                    .collect(),
            },
        }
    }

    fn bind_selector(&self, selector: Selector) -> EvalSelector {
        match selector {
            Selector::Wildcard => EvalSelector::Wildcard,
            Selector::Identifier { name } => EvalSelector::Identifier { name },
            Selector::Index { index } => EvalSelector::Index { index },
            Selector::Slice { start, end, step } => EvalSelector::Slice { start, end, step },
        }
    }
}
