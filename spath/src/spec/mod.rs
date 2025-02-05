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

//! Types representing the concepts of RFC 9535.

use crate::ConcreteVariantArray;
use crate::ConcreteVariantObject;
use crate::VariantValue;

pub mod functions;
pub mod query;
pub mod segment;
pub mod selector;

/// §2.3.2.2 (Wildcard Selector) Semantics
///
/// A wildcard selector selects the nodes of all children of an object or array.
///
/// Note that the children of an object are its member values, not its member names.
///
/// The wildcard selector selects nothing from a primitive variant value.
fn select_wildcard<'b, T: VariantValue>(result: &mut Vec<&'b T>, current: &'b T) {
    if let Some(list) = current.as_array() {
        for v in list.iter() {
            result.push(v);
        }
    } else if let Some(obj) = current.as_object() {
        for v in obj.values() {
            result.push(v);
        }
    }
}
