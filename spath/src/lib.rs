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

//! # SPath: Query expressions for semi-structured data

#![cfg_attr(docsrs, feature(doc_auto_cfg))]

mod node;
pub use node::*;

mod path;
pub use path::*;

mod spath;
pub use spath::*;

pub mod spec;

mod value;
pub use value::*;

#[cfg(feature = "json")]
mod json;
mod parser;
#[cfg(feature = "toml")]
mod toml;

/// An error that can occur during parsing the SPath query.
#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub struct ParseError(pub String);
