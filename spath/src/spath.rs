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

use crate::parser::ast::{Segment, Selector};
use crate::parser::runner::run_parser;
use crate::ParseError;
use crate::Value;

#[derive(Debug, Clone)]
pub struct SPath {
    segments: Vec<EvalSegment>,
}

impl SPath {
    pub fn eval(&self, value: Value) -> Option<Value> {
        None
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

#[derive(Debug)]
pub struct Binder {
    segments: Vec<Segment>,
}

impl Binder {
    /// Create a new binder by parsing the spath expression.
    pub fn parse(source: &str) -> Result<Self, ParseError> {
        let segments = run_parser(source)?;
        Ok(Self { segments })
    }

    /// Bind the segments to the eval context.
    pub fn bind(&self) -> SPath {
        let segments = self.segments.iter().map(|s| self.bind_segment(s)).collect();
        SPath { segments }
    }

    fn bind_segment(&self, segment: &Segment) -> EvalSegment {
        match segment {
            Segment::Child { selectors } => EvalSegment::Child {
                selectors: selectors.iter().map(|s| self.bind_selector(s)).collect(),
            },
            Segment::Descendant { selectors } => EvalSegment::Descendant {
                selectors: selectors.iter().map(|s| self.bind_selector(s)).collect(),
            },
        }
    }

    fn bind_selector(&self, selector: &Selector) -> EvalSelector {
        match selector {
            Selector::Wildcard => EvalSelector::Wildcard,
            Selector::Identifier { name } => EvalSelector::Identifier { name: name.clone() },
            Selector::Index { index } => EvalSelector::Index { index: *index },
            Selector::Slice { start, end, step } => EvalSelector::Slice {
                start: *start,
                end: *end,
                step: *step,
            },
        }
    }
}
