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
                let mut result = value;
                for selector in selectors {
                    if let Some(res) = selector.eval(root, result) {
                        result = res;
                    } else {
                        return None;
                    }
                }
                Some(result)
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
    Slice {
        /// The start index of the slice, inclusive. Default to 0.
        start: i64,
        /// The end index of the slice, exclusive. Default to the length of the array.
        end: Option<i64>,
        /// The step to iterate the slice. Default to 1.
        step: i64,
    },
}

impl EvalSelector {
    fn eval(&self, _root: &Value, value: Value) -> Option<Value> {
        match self {
            EvalSelector::Wildcard => Some(value),
            EvalSelector::Identifier { name } => {
                if let Value::Object(map) = value {
                    map.get(name).cloned()
                } else {
                    None
                }
            }
            EvalSelector::Index { index } => {
                todo!("index selector: {index}")
            }
            EvalSelector::Slice { start, end, step } => {
                todo!("slice selector: {start}, {end:?}, {step}")
            }
        }
    }
}

#[derive(Debug, Clone)]
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
