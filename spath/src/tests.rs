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
use googletest::prelude::eq;
use insta::assert_debug_snapshot;

use crate::json_testdata;
use crate::SPath;
use crate::Value;

fn eval_spath(spath: &str, value: &Value) -> Option<Value> {
    let spath = SPath::new(spath).unwrap();
    spath.eval(value)
}

#[test]
fn test_root_wildcard() {
    let value = json_testdata("rfc-9535-example-1.json");
    let value = Value::from(value);

    let result = eval_spath("$", &value).unwrap();
    assert_that!(result, eq(&value));
    let result = eval_spath("$.*", &value).unwrap();
    assert_that!(result, eq(&value));
    let result = eval_spath("$[*]", &value).unwrap();
    assert_that!(result, eq(&value));
}

#[test]
fn test_basic_name_selector() {
    let value = json_testdata("rfc-9535-example-1.json");
    let value = Value::from(value);

    let result = eval_spath(r#"$["store"]['bicycle']"#, &value).unwrap();
    assert_debug_snapshot!(result, @r#"{"color":'red',"price":399}"#);
    let result = eval_spath(r#"$.store.bicycle.color"#, &value).unwrap();
    assert_debug_snapshot!(result, @"'red'");
    let result = eval_spath(r#"$.store.book.*"#, &value).unwrap();
    assert_debug_snapshot!(result, @r#"[{"author":'Nigel Rees',"category":'reference',"price":8.95,"title":'Sayings of the Century'},{"author":'Evelyn Waugh',"category":'fiction',"price":12.99,"title":'Sword of Honour'},{"author":'Herman Melville',"category":'fiction',"isbn":'0-553-21311-3',"price":8.99,"title":'Moby Dick'},{"author":'J. R. R. Tolkien',"category":'fiction',"isbn":'0-395-19395-8',"price":22.99,"title":'The Lord of the Rings'}]"#);
}

#[test]
fn test_basic_index_slice_selector() {
    let value = json_testdata("rfc-9535-example-4.json");
    let value = Value::from(value);

    // §2.3.4.3 (Example) Table 7: Index Selector Examples
    let result = eval_spath(r#"$[1]"#, &value).unwrap();
    assert_debug_snapshot!(result, @"'b'");
    let result = eval_spath(r#"$[0]"#, &value).unwrap();
    assert_debug_snapshot!(result, @"'a'");
}

#[test]
fn test_basic_array_slice_selector() {
    let value = json_testdata("rfc-9535-example-5.json");
    let value = Value::from(value);

    // §2.3.4.3 (Example) Table 9: Array Slice Selector Examples
    let result = eval_spath(r#"$[1:3]"#, &value).unwrap();
    assert_debug_snapshot!(result, @"['b','c']");
    let result = eval_spath(r#"$[5:]"#, &value).unwrap();
    assert_debug_snapshot!(result, @"['f','g']");
    let result = eval_spath(r#"$[1:5:2]"#, &value).unwrap();
    assert_debug_snapshot!(result, @"['b','d']");
    let result = eval_spath(r#"$[5:1:-2]"#, &value).unwrap();
    assert_debug_snapshot!(result, @"['f','d']");
    let result = eval_spath(r#"$[::-1]"#, &value).unwrap();
    assert_debug_snapshot!(result, @"['g','f','e','d','c','b','a']");
}

#[test]
fn test_basic_child_and_descendant_segment() {
    let value = json_testdata("rfc-9535-example-8.json");
    let value = Value::from(value);

    // §2.5.1.3 (Example) Table 15: Child Segment Examples
    let result = eval_spath(r#"$[0, 3]"#, &value).unwrap();
    assert_debug_snapshot!(result, @"['a','d']");
    let result = eval_spath(r#"$[0:2, 5]"#, &value).unwrap();
    assert_debug_snapshot!(result, @"[['a','b'],'f']");
    let result = eval_spath(r#"$[0,0]"#, &value).unwrap();
    assert_debug_snapshot!(result, @"['a','a']");

    let value = json_testdata("rfc-9535-example-9.json");
    let value = Value::from(value);
    // §2.5.2.3 (Example) Table 16: Descendant Segment Examples
    let result = eval_spath(r#"$..j"#, &value).unwrap();
    assert_debug_snapshot!(result, @"[1,4]");
    let result = eval_spath(r#"$..[0]"#, &value).unwrap();
    assert_debug_snapshot!(result, @r#"[5,{"j":4}]"#);
    let result = eval_spath(r#"$..*"#, &value).unwrap();
    assert_debug_snapshot!(result, @r#"[{"a":[5,3,[{"j":4},{"k":6}]],"o":{"j":1,"k":2}},{"j":1,"k":2},2,1,[5,3,[{"j":4},{"k":6}]],[{"j":4},{"k":6}],{"k":6},6,{"j":4},4,3,5]"#);
    let result = eval_spath(r#"$..[*]"#, &value).unwrap();
    assert_debug_snapshot!(result, @r#"[{"a":[5,3,[{"j":4},{"k":6}]],"o":{"j":1,"k":2}},{"j":1,"k":2},2,1,[5,3,[{"j":4},{"k":6}]],[{"j":4},{"k":6}],{"k":6},6,{"j":4},4,3,5]"#);
    let result = eval_spath(r#"$..o"#, &value).unwrap();
    assert_debug_snapshot!(result, @r#"[{"j":1,"k":2}]"#);
    let result = eval_spath(r#"$.o..[*, *]"#, &value).unwrap();
    assert_debug_snapshot!(result, @r#"[{"j":1,"k":2},{"j":1,"k":2},2,2,1,1]"#);
    let result = eval_spath(r#"$.a..[0, 1]"#, &value).unwrap();
    assert_debug_snapshot!(result, @r#"[5,3,{"j":4},{"k":6}]"#);
}
