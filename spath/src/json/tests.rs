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

use googletest::assert_that;
use googletest::matchers::eq;
use insta::assert_snapshot;
use serde_json::Value as JsonValue;

use crate::json_testdata;
use crate::Value;

fn assert_testdata_identical(path: &str) -> String {
    let json_value = json_testdata(path);
    let value = Value::from(json_value.clone());
    assert_that!(json_value, eq(&JsonValue::from(value.clone())));
    format!("{:?}", value)
}

#[test]
fn test_rfc_9535_example_conversion() {
    // §1.5 Figure 1
    assert_snapshot!(assert_testdata_identical("rfc-9535-example-1.json"));
    // §2.3.1.3 Example
    assert_snapshot!(assert_testdata_identical("rfc-9535-example-2.json"));
    // §2.3.2.3 Example
    assert_snapshot!(assert_testdata_identical("rfc-9535-example-3.json"));
    // §2.3.3.3 Example
    assert_snapshot!(assert_testdata_identical("rfc-9535-example-4.json"));
    // §2.3.4.3 Example
    assert_snapshot!(assert_testdata_identical("rfc-9535-example-5.json"));
    // §2.3.5.3 Example 1
    assert_snapshot!(assert_testdata_identical("rfc-9535-example-6.json"));
    // §2.3.5.3 Example 2
    assert_snapshot!(assert_testdata_identical("rfc-9535-example-7.json"));
    // §2.5.1.3 Example 1
    assert_snapshot!(assert_testdata_identical("rfc-9535-example-8.json"));
    // §2.5.2.3 Example 2
    assert_snapshot!(assert_testdata_identical("rfc-9535-example-9.json"));
    // §2.6.1 Example
    assert_snapshot!(assert_testdata_identical("rfc-9535-example-10.json"));
}
