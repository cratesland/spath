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

//! Types representing the different selectors in SPath.

pub mod filter;
pub mod index;
pub mod name;
pub mod slice;

use std::fmt;

/// An SPath selector
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Selector {}

impl fmt::Display for Selector {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("implement Selector Display")
    }
}
