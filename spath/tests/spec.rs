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

#![cfg(feature = "json")]

//! Spec tests based on RFC 9535.

mod common;

use common::manifest_dir;
use googletest::assert_that;
use googletest::matchers::container_eq;
use googletest::matchers::none;
use googletest::prelude::eq;
use googletest::prelude::some;
use insta::assert_compact_json_snapshot;
use serde_json::json;
use spath::NodeList;
use spath::SPath;

fn json_testdata(filename: &str) -> serde_json::Value {
    let path = manifest_dir().join("testdata").join(filename);
    let content = std::fs::read_to_string(path).unwrap();
    serde_json::from_str(&content).unwrap()
}

fn eval_spath<'a>(
    spath: &str,
    value: &'a serde_json::Value,
) -> Result<NodeList<'a, serde_json::Value>, spath::ParseError> {
    let registry = spath::json::BuiltinFunctionRegistry::default();
    let spath = SPath::parse_with_registry(spath, registry)?;
    Ok(spath.query(value))
}

#[test]
fn test_root_identical() {
    let value = json_testdata("rfc-9535-example-1.json");
    let result = eval_spath("$", &value).unwrap();
    let result = result.exactly_one().unwrap();
    assert_that!(result, eq(&value));
}

#[test]
fn test_basic_name_selector() {
    let value = json_testdata("rfc-9535-example-1.json");
    let result = eval_spath(r#"$["store"]['bicycle']"#, &value).unwrap();
    let result = result.exactly_one().unwrap();
    assert_compact_json_snapshot!(result, @r#"{"color": "red", "price": 399}"#);
    let result = eval_spath(r#"$.store.bicycle.color"#, &value).unwrap();
    let result = result.exactly_one().unwrap();
    assert_compact_json_snapshot!(result, @r#""red""#);
    let result = eval_spath(r#"$.store.book.*"#, &value).unwrap();
    let result = result.all();
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
    let result = result.exactly_one().unwrap();
    assert_compact_json_snapshot!(result, @r#"{"k.k": 3}"#);
    let result = eval_spath(r#"$.o['j j']['k.k']	"#, &value).unwrap();
    let result = result.exactly_one().unwrap();
    assert_compact_json_snapshot!(result, @"3");
    let result = eval_spath(r#"$.o["j j"]["k.k"]"#, &value).unwrap();
    let result = result.exactly_one().unwrap();
    assert_compact_json_snapshot!(result, @"3");
    let result = eval_spath(r#"$["'"]["@"]"#, &value).unwrap();
    let result = result.exactly_one().unwrap();
    assert_compact_json_snapshot!(result, @"2");
}

#[test]
fn test_basic_wildcard_selector() {
    // §2.3.2.3 (Example) Table 6: Wildcard Selector Examples
    let value = json_testdata("rfc-9535-example-3.json");
    let result = eval_spath(r#"$[*]"#, &value).unwrap();
    let result = result.all();
    assert_compact_json_snapshot!(result, @r#"[[5, 3], {"j": 1, "k": 2}]"#);
    let result = eval_spath(r#"$.o[*]"#, &value).unwrap();
    let result = result.all();
    assert_compact_json_snapshot!(result, @"[1, 2]");
    let result = eval_spath(r#"$.o[*, *]"#, &value).unwrap();
    let result = result.all();
    assert_compact_json_snapshot!(result, @"[1, 2, 1, 2]");
    let result = eval_spath(r#"$.a[*]"#, &value).unwrap();
    let result = result.all();
    assert_compact_json_snapshot!(result, @"[5, 3]");
}

#[test]
fn test_basic_index_slice_selector() {
    // §2.3.3.3 (Example) Table 7: Index Selector Examples
    let value = json_testdata("rfc-9535-example-4.json");
    let result = eval_spath(r#"$[1]"#, &value).unwrap();
    let result = result.exactly_one().unwrap();
    assert_compact_json_snapshot!(result, @r#""b""#);
    let result = eval_spath(r#"$[0]"#, &value).unwrap();
    let result = result.exactly_one().unwrap();
    assert_compact_json_snapshot!(result, @r#""a""#);
}

#[test]
fn test_basic_array_slice_selector() {
    // §2.3.4.3 (Example) Table 9: Array Slice Selector Examples
    let value = json_testdata("rfc-9535-example-5.json");
    let result = eval_spath(r#"$[1:3]"#, &value).unwrap();
    let result = result.all();
    assert_compact_json_snapshot!(result, @r#"["b", "c"]"#);
    let result = eval_spath(r#"$[5:]"#, &value).unwrap();
    let result = result.all();
    assert_compact_json_snapshot!(result, @r#"["f", "g"]"#);
    let result = eval_spath(r#"$[1:5:2]"#, &value).unwrap();
    let result = result.all();
    assert_compact_json_snapshot!(result, @r#"["b", "d"]"#);
    let result = eval_spath(r#"$[5:1:-2]"#, &value).unwrap();
    let result = result.all();
    assert_compact_json_snapshot!(result, @r#"["f", "d"]"#);
    let result = eval_spath(r#"$[::-1]"#, &value).unwrap();
    let result = result.all();
    assert_compact_json_snapshot!(result, @r#"["g", "f", "e", "d", "c", "b", "a"]"#);
}

#[test]
fn test_basic_child_and_descendant_segment() {
    // §2.5.1.3 (Example) Table 15: Child Segment Examples
    let value = json_testdata("rfc-9535-example-8.json");
    let result = eval_spath(r#"$[0, 3]"#, &value).unwrap();
    let result = result.all();
    assert_compact_json_snapshot!(result, @r#"["a", "d"]"#);
    let result = eval_spath(r#"$[0:2, 5]"#, &value).unwrap();
    let result = result.all();
    assert_compact_json_snapshot!(result, @r#"["a", "b", "f"]"#);
    let result = eval_spath(r#"$[0,0]"#, &value).unwrap();
    let result = result.all();
    assert_compact_json_snapshot!(result, @r#"["a", "a"]"#);

    // §2.5.2.3 (Example) Table 16: Descendant Segment Examples
    let value = json_testdata("rfc-9535-example-9.json");
    let result = eval_spath(r#"$..j"#, &value).unwrap();
    let result = result.all();
    assert_compact_json_snapshot!(result, @"[4, 1]");
    let result = eval_spath(r#"$..[0]"#, &value).unwrap();
    let result = result.all();
    assert_compact_json_snapshot!(result, @r#"[5, {"j": 4}]"#);
    let result = eval_spath(r#"$..*"#, &value).unwrap();
    let result = result.all();
    assert_compact_json_snapshot!(result, @r#"[[5, 3, [{"j": 4}, {"k": 6}]], {"j": 1, "k": 2}, 5, 3, [{"j": 4}, {"k": 6}], {"j": 4}, {"k": 6}, 4, 6, 1, 2]"#);
    let result = eval_spath(r#"$..[*]"#, &value).unwrap();
    let result = result.all();
    assert_compact_json_snapshot!(result, @r#"[[5, 3, [{"j": 4}, {"k": 6}]], {"j": 1, "k": 2}, 5, 3, [{"j": 4}, {"k": 6}], {"j": 4}, {"k": 6}, 4, 6, 1, 2]"#);
    let result = eval_spath(r#"$..o"#, &value).unwrap();
    let result = result.all();
    assert_compact_json_snapshot!(result, @r#"[{"j": 1, "k": 2}]"#);
    let result = eval_spath(r#"$.o..[*, *]"#, &value).unwrap();
    let result = result.all();
    assert_compact_json_snapshot!(result, @"[1, 2, 1, 2]");
    let result = eval_spath(r#"$.a..[0, 1]"#, &value).unwrap();
    let result = result.all();
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
    let array_of_null = [null.clone()];
    let value_of_ident_null = serde_json::Value::Number(1i64.into());

    let result = eval_spath(r#"$.a"#, &value).unwrap();
    assert_that!(result.at_most_one().unwrap(), some(eq(&null)));
    let result = eval_spath(r#"$.a[0]"#, &value).unwrap();
    assert_that!(result.at_most_one().unwrap(), none());
    let result = eval_spath(r#"$.a.d"#, &value).unwrap();
    assert_that!(result.at_most_one().unwrap(), none());
    let result = eval_spath(r#"$.b[0]"#, &value).unwrap();
    assert_that!(result.at_most_one().unwrap(), some(eq(&null)));
    let result = eval_spath(r#"$.b[*]"#, &value).unwrap();
    assert_that!(
        result.all(),
        container_eq(array_of_null.iter().collect::<Vec<_>>())
    );
    let result = eval_spath(r#"$.null"#, &value).unwrap();
    assert_that!(
        result.at_most_one().unwrap(),
        some(eq(&value_of_ident_null))
    );
}

#[test]
fn test_filters() {
    // §2.3.5.3 Table 12: Filter Selector Examples
    let value = json! {{
      "a": [3, 5, 1, 2, 4, 6,
            {"b": "j"},
            {"b": "k"},
            {"b": {}},
            {"b": "kilo"}
           ],
      "o": {"p": 1, "q": 2, "r": 3, "s": 5, "t": {"u": 6}},
      "e": "f"
    }};

    let result = eval_spath(r#"$.a[?@.b == 'kilo']"#, &value).unwrap();
    let result = result.exactly_one().unwrap();
    assert_compact_json_snapshot!(result, @r#"{"b": "kilo"}"#);
    let result = eval_spath(r#"$.a[?(@.b == 'kilo')]"#, &value).unwrap();
    let result = result.exactly_one().unwrap();
    assert_compact_json_snapshot!(result, @r#"{"b": "kilo"}"#);
    let result = eval_spath(r#"$.a[?@>3.5]"#, &value).unwrap();
    let result = result.all();
    assert_compact_json_snapshot!(result, @r#"[5, 4, 6]"#);
    let result = eval_spath(r#"$.a[?@.b]"#, &value).unwrap();
    let result = result.all();
    assert_compact_json_snapshot!(result, @r#"[{"b": "j"}, {"b": "k"}, {"b": {}}, {"b": "kilo"}]"#);
}
#[test]
fn test_filter_functions() {
    let values = json! {{
        "a": [1, 2, 3, 4, 5],
        "b": 42,
        "c": [],
        "d": {"e": 1, "f": 2},
    }};

    let result = eval_spath(r#"$[?length(@) < 3]"#, &values).unwrap();
    let result = result.all();
    assert_compact_json_snapshot!(result, @r#"[[], {"e": 1, "f": 2}]"#);
    let result = eval_spath(r#"$[?count(@.*) > 1]"#, &values).unwrap();
    let result = result.all();
    assert_compact_json_snapshot!(result, @r#"[[1, 2, 3, 4, 5], {"e": 1, "f": 2}]"#);
}

#[test]
#[cfg(feature = "regex")]
fn test_regex_functions() {
    let values = json! {[
        {"timezone": "UTC", "offset": 0},
        {"timezone": "CET", "offset": 1},
        {"timezone": "PST", "offset": -8},
        {"timezone": "JST", "offset": 9},
    ]};
    let result = eval_spath(r#"$[?match(@.timezone, "...")].offset"#, &values).unwrap();
    let result = result.all();
    assert_compact_json_snapshot!(result, @"[0, 1, -8, 9]");
    let result = eval_spath(r#"$[?match(@.timezone, "..")].offset"#, &values).unwrap();
    let result = result.all();
    assert_compact_json_snapshot!(result, @"[]");
    let result = eval_spath(r#"$[?search(@.timezone, "ST")].offset"#, &values).unwrap();
    let result = result.all();
    assert_compact_json_snapshot!(result, @"[-8, 9]");
}
