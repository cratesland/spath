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
//! Here is a quick example that shows how to use the `spath` crate to query
//! JSONPath alike expression over JSON data:
//!
//! ```rust
//! # #[cfg(feature = "json")]
//! # {
//! use serde_json::json;
//! use serde_json::Value as JsonValue;
//! use spath::SPath;
//! use spath::Value;
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
//! let value = Value::from(data);
//! let result = spath.eval(&value).unwrap();
//! assert_eq!(JsonValue::from(result), json!("+44 2345678"));
//! # }
//! ```
//!
//! ## Feature flags
//!
//! - `json`: Enabled conversion between `serde_json::Value` and [`Value`].
//! - `serde`: Implement `serde::Serialize` for [`Number`] and [`Value`], plus `serde::Deserialize`
//!   for [`Number`].

mod value;
pub use value::*;

mod error;
pub use error::*;

mod spath;
pub use spath::*;

#[cfg(any(feature = "json", test))]
mod json;
mod parser;
#[cfg(feature = "serde")]
mod serde;

#[cfg(test)]
mod tests;

#[cfg(test)]
fn manifest_dir() -> std::path::PathBuf {
    let dir = env!("CARGO_MANIFEST_DIR");
    std::path::PathBuf::from(dir).canonicalize().unwrap()
}

#[cfg(test)]
fn json_testdata(filename: &str) -> serde_json::Value {
    let path = manifest_dir().join("testdata").join(filename);
    let content = std::fs::read_to_string(path).unwrap();
    serde_json::from_str(&content).unwrap()
}
