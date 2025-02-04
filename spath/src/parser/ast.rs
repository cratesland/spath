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

#[derive(Debug, Clone)]
pub struct RootPathQuery {
    pub segments: Vec<Segment>,
}

#[derive(Debug, Clone)]
pub enum Segment {
    /// §2.5.1 Child Segment.
    Child {
        /// The selectors of the child segment.
        selectors: Vec<Selector>,
    },
    /// §2.5.2 Descendant Segment.
    Descendant {
        /// The selectors of the descendant segment.
        selectors: Vec<Selector>,
    },
}

#[derive(Debug, Clone)]
pub enum Selector {
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
