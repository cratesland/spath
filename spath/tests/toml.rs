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

mod common;

use common::manifest_dir;
use googletest::assert_that;
use googletest::matchers::eq;
use insta::assert_compact_json_snapshot;
use spath::NodeList;
use spath::ParseError;
use spath::SPath;
use toml::Value;

fn toml_testdata(filename: &str) -> Value {
    let path = manifest_dir().join("testdata").join(filename);
    let content = std::fs::read_to_string(path).unwrap();
    toml::from_str(&content).unwrap()
}

fn eval_spath<'a>(spath: &str, value: &'a Value) -> Result<NodeList<'a, Value>, ParseError> {
    let spath = SPath::parse_with_registry(spath, spath::toml::BuiltinFunctionRegistry)?;
    Ok(spath.query(value))
}

#[test]
fn test_root_identical() {
    let value = toml_testdata("learn-toml-in-y-minutes.toml");
    let result = eval_spath("$", &value).unwrap();
    let result = result.exactly_one().unwrap();
    assert_that!(result, eq(&value));
}

#[test]
fn test_casual() {
    let value = toml_testdata("learn-toml-in-y-minutes.toml");
    let result = eval_spath(r#"$..["name"]"#, &value).unwrap();
    let result = result.all();
    assert_compact_json_snapshot!(result, @r#"["array of table", "Nail"]"#);
    let result = eval_spath(r#"$..[1]"#, &value).unwrap();
    let result = result.all();
    assert_compact_json_snapshot!(result, @r#"[2, "are", "different", ["all", "strings", "are the same", "type"], 2.4, "strings", "is", {}]"#);
}
