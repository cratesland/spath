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

use insta::assert_debug_snapshot;

use crate::Value;

#[cfg(feature = "json")]
#[test]
fn test_serde_json_to_variant() {
    let value: serde_json::Value = serde_json::from_str(include_str!("simple.json")).unwrap();
    let value = Value::from(value);
    assert_debug_snapshot!(value);
}
