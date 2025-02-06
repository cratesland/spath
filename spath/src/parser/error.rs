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

use crate::parser::range::Range;

/// Error type for SPath query string parsing errors
#[derive(Debug, thiserror::Error)]
#[error("{message}")]
pub struct ParseError {
    range: Range,
    message: String,
}

impl ParseError {
    pub fn new(range: Range, message: impl Into<String>) -> Self {
        let message = message.into();
        Self { range, message }
    }
}
