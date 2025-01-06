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
