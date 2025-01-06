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
//!
//! You can use it as a drop-in replacement for JSONPath, but also for other semi-structured data
//! formats like TOML or user-defined variants.
//!
//! ## Examples
//!
//! ### JSON
//!
//! Here is a quick example that shows how to use the `spath` crate to query
//! JSONPath alike expression over JSON data:
//!
//! ```rust
//! # #[cfg(feature = "json")]
//! # {
//! use serde_json::json;
//! use serde_json::Value;
//! use spath::SPath;
//! use spath::VariantValue;
//!
//! let data = json!({
//!   "name": "John Doe",
//!   "age": 43,
//!   "phones": [
//!     "+44 1234567",
//!     "+44 2345678"
//!   ]
//! });
//!
//! let spath = SPath::new("$.phones[1]").unwrap();
//! let result = spath.eval(&data).unwrap();
//! assert_eq!(result, json!("+44 2345678"));
//! # }
//! ```
//!
//! ### TOML
//!
//! Here is a quick example that shows how to use the `spath` crate to query
//! JSONPath alike expression over TOML data:
//!
//! ```rust
//! # #[cfg(feature = "toml")]
//! # {
//! use spath::SPath;
//! use spath::VariantValue;
//! use toml::Value;
//!
//! let data = r#"
//! [package]
//! name = "spath"
//! version = "0.1.0"
//! authors = ["tison"]
//!
//! [dependencies]
//! serde = "1.0"
//! "#;
//!
//! let data = data.parse::<Value>().unwrap();
//! let spath = SPath::new("$.package.name").unwrap();
//! let result = spath.eval(&data).unwrap();
//! assert_eq!(result, Value::String("spath".to_string()));
//! # }
//! ```
//!
//! ## Feature flags
//!
//! - `json`: impl [`VariantValue`] for `serde_json::Value`.
//! - `toml`: impl [`VariantValue`] for `toml::Value`.

mod value;
pub use value::*;

mod error;
pub use error::*;

mod spath;
pub use spath::*;

#[cfg(any(feature = "json", test))]
mod json;
mod parser;
#[cfg(feature = "toml")]
mod toml;

#[cfg(test)]
mod tests;

#[cfg(test)]
fn manifest_dir() -> std::path::PathBuf {
    let dir = env!("CARGO_MANIFEST_DIR");
    std::path::PathBuf::from(dir).canonicalize().unwrap()
}
