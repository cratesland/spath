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

//! Spec tmp_hack_no_tests based on RFC 9535.

mod common;
use common::manifest_dir;
use googletest::assert_that;
use googletest::matchers::none;
use googletest::prelude::eq;
use googletest::prelude::some;
use insta::assert_compact_json_snapshot;
use spath::SPath;
use spath::VariantValue;

fn json_testdata(filename: &str) -> serde_json::Value {
    let path = manifest_dir().join("testdata").join(filename);
    let content = std::fs::read_to_string(path).unwrap();
    serde_json::from_str(&content).unwrap()
}

fn eval_spath<T: VariantValue>(spath: &str, value: &T) -> Option<T> {
    let spath = SPath::new(spath).unwrap();
    spath.eval(value)
}

#[test]
fn test_root_identical() {
    let value = json_testdata("rfc-9535-example-1.json");
    let result = eval_spath("$", &value).unwrap();
    assert_that!(result, eq(&value));
}

#[test]
fn test_basic_name_selector() {
    let value = json_testdata("rfc-9535-example-1.json");
    let result = eval_spath(r#"$["store"]['bicycle']"#, &value).unwrap();
    assert_compact_json_snapshot!(result, @r#"{"color": "red", "price": 399}"#);
    let result = eval_spath(r#"$.store.bicycle.color"#, &value).unwrap();
    assert_compact_json_snapshot!(result, @r#""red""#);
    let result = eval_spath(r#"$.store.book.*"#, &value).unwrap();
    assert_compact_json_snapshot!(result, @r#"
    [
      {
        "author": "Nigel Rees",
        "category": "reference",
        "price": 8.95,
        "title": "Sayings of the Century"
      },
      {
        "author": "Evelyn Waugh",
        "category": "fiction",
        "price": 12.99,
        "title": "Sword of Honour"
      },
      {
        "author": "Herman Melville",
        "category": "fiction",
        "isbn": "0-553-21311-3",
        "price": 8.99,
        "title": "Moby Dick"
      },
      {
        "author": "J. R. R. Tolkien",
        "category": "fiction",
        "isbn": "0-395-19395-8",
        "price": 22.99,
        "title": "The Lord of the Rings"
      }
    ]
    "#);

    // §2.3.1.3 (Example) Table 5: Name Selector Examples
    let value = json_testdata("rfc-9535-example-2.json");
    let result = eval_spath(r#"$.o['j j']"#, &value).unwrap();
    assert_compact_json_snapshot!(result, @r#"{"k.k": 3}"#);
    let result = eval_spath(r#"$.o['j j']['k.k']	"#, &value).unwrap();
    assert_compact_json_snapshot!(result, @"3");
    let result = eval_spath(r#"$.o["j j"]["k.k"]"#, &value).unwrap();
    assert_compact_json_snapshot!(result, @"3");
    let result = eval_spath(r#"$["'"]["@"]"#, &value).unwrap();
    assert_compact_json_snapshot!(result, @"2");
}

#[test]
fn test_basic_wildcard_selector() {
    // §2.3.2.3 (Example) Table 6: Wildcard Selector Examples
    let value = json_testdata("rfc-9535-example-3.json");
    let result = eval_spath(r#"$[*]"#, &value).unwrap();
    assert_compact_json_snapshot!(result, @r#"[[5, 3], {"j": 1, "k": 2}]"#);
    let result = eval_spath(r#"$.o[*]"#, &value).unwrap();
    assert_compact_json_snapshot!(result, @"[1, 2]");
    let result = eval_spath(r#"$.o[*, *]"#, &value).unwrap();
    assert_compact_json_snapshot!(result, @"[[1, 2], [1, 2]]");
    let result = eval_spath(r#"$.a[*]"#, &value).unwrap();
    assert_compact_json_snapshot!(result, @"[5, 3]");
}

#[test]
fn test_basic_index_slice_selector() {
    // §2.3.3.3 (Example) Table 7: Index Selector Examples
    let value = json_testdata("rfc-9535-example-4.json");
    let result = eval_spath(r#"$[1]"#, &value).unwrap();
    assert_compact_json_snapshot!(result, @r#""b""#);
    let result = eval_spath(r#"$[0]"#, &value).unwrap();
    assert_compact_json_snapshot!(result, @r#""a""#);
}

#[test]
fn test_basic_array_slice_selector() {
    // §2.3.4.3 (Example) Table 9: Array Slice Selector Examples
    let value = json_testdata("rfc-9535-example-5.json");
    let result = eval_spath(r#"$[1:3]"#, &value).unwrap();
    assert_compact_json_snapshot!(result, @r#"["b", "c"]"#);
    let result = eval_spath(r#"$[5:]"#, &value).unwrap();
    assert_compact_json_snapshot!(result, @r#"["f", "g"]"#);
    let result = eval_spath(r#"$[1:5:2]"#, &value).unwrap();
    assert_compact_json_snapshot!(result, @r#"["b", "d"]"#);
    let result = eval_spath(r#"$[5:1:-2]"#, &value).unwrap();
    assert_compact_json_snapshot!(result, @r#"["f", "d"]"#);
    let result = eval_spath(r#"$[::-1]"#, &value).unwrap();
    assert_compact_json_snapshot!(result, @r#"["g", "f", "e", "d", "c", "b", "a"]"#);
}

#[test]
fn test_basic_child_and_descendant_segment() {
    // §2.5.1.3 (Example) Table 15: Child Segment Examples
    let value = json_testdata("rfc-9535-example-8.json");
    let result = eval_spath(r#"$[0, 3]"#, &value).unwrap();
    assert_compact_json_snapshot!(result, @r#"["a", "d"]"#);
    let result = eval_spath(r#"$[0:2, 5]"#, &value).unwrap();
    assert_compact_json_snapshot!(result, @r#"[["a", "b"], "f"]"#);
    let result = eval_spath(r#"$[0,0]"#, &value).unwrap();
    assert_compact_json_snapshot!(result, @r#"["a", "a"]"#);

    // §2.5.2.3 (Example) Table 16: Descendant Segment Examples
    let value = json_testdata("rfc-9535-example-9.json");
    let result = eval_spath(r#"$..j"#, &value).unwrap();
    assert_compact_json_snapshot!(result, @"[1, 4]");
    let result = eval_spath(r#"$..[0]"#, &value).unwrap();
    assert_compact_json_snapshot!(result, @r#"[5, {"j": 4}]"#);
    let result = eval_spath(r#"$..*"#, &value).unwrap();
    assert_compact_json_snapshot!(result, @r#"[[[5, 3, [{"j": 4}, {"k": 6}]], {"j": 1, "k": 2}], [1, 2], [5, 3, [{"j": 4}, {"k": 6}]], [{"j": 4}, {"k": 6}], [6], [4]]"#);
    let result = eval_spath(r#"$..[*]"#, &value).unwrap();
    assert_compact_json_snapshot!(result, @r#"[[[5, 3, [{"j": 4}, {"k": 6}]], {"j": 1, "k": 2}], [1, 2], [5, 3, [{"j": 4}, {"k": 6}]], [{"j": 4}, {"k": 6}], [6], [4]]"#);
    let result = eval_spath(r#"$..o"#, &value).unwrap();
    assert_compact_json_snapshot!(result, @r#"[{"j": 1, "k": 2}]"#);
    let result = eval_spath(r#"$.o..[*, *]"#, &value).unwrap();
    assert_compact_json_snapshot!(result, @"[[1, 2], [1, 2]]");
    let result = eval_spath(r#"$.a..[0, 1]"#, &value).unwrap();
    assert_compact_json_snapshot!(result, @r#"[5, 3, {"j": 4}, {"k": 6}]"#);
}

#[test]
fn test_basic_null_semantic() {
    // §2.6 Semantics of null
    //
    // JSON null is treated the same as any other JSON value, i.e.,
    // it is not taken to mean "undefined" or "missing".

    // §2.6.1 (Example) Table 17: Examples Involving (or Not Involving) null
    let value = json_testdata("rfc-9535-example-10.json");
    let null = serde_json::Value::Null;
    let array_of_null = serde_json::Value::Array(vec![null.clone()]);
    let value_of_ident_null = serde_json::Value::Number(1i64.into());
    assert_that!(eval_spath(r#"$.a"#, &value), some(eq(&null)));
    assert_that!(eval_spath(r#"$.a[0]"#, &value), none());
    assert_that!(eval_spath(r#"$.a.d"#, &value), none());
    assert_that!(eval_spath(r#"$.b[0]"#, &value), some(eq(&null)));
    assert_that!(eval_spath(r#"$.b[*]"#, &value), some(eq(&array_of_null)));
    assert_that!(
        eval_spath(r#"$.null"#, &value),
        some(eq(&value_of_ident_null))
    );
}
