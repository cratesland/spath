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

use insta::assert_compact_json_snapshot;
use spath::SPath;
use toml::Value;
mod common;
use common::manifest_dir;

fn toml_testdata(filename: &str) -> Value {
    let path = manifest_dir().join("testdata").join(filename);
    let content = std::fs::read_to_string(path).unwrap();
    toml::from_str(&content).unwrap()
}

fn eval_spath(spath: &str, value: &Value) -> Option<Value> {
    let spath = SPath::new(spath).unwrap();
    spath.eval(value)
}

#[test]
fn test_root_identical() {
    let value = toml_testdata("learn-toml-in-y-minutes.toml");
    let result = eval_spath("$", &value).unwrap();
    assert_eq!(result, value);
}

#[test]
fn test_casual() {
    let value = toml_testdata("learn-toml-in-y-minutes.toml");
    let result = eval_spath(r#"$..["name"]"#, &value).unwrap();
    assert_compact_json_snapshot!(result, @r#"["Nail", "array of table"]"#);
    let result = eval_spath(r#"$..[1]"#, &value).unwrap();
    assert_compact_json_snapshot!(result, @r#"[{}, "is", ["all", "strings", "are the same", "type"], "strings", 2.4, "different", "are", 2]"#);
}
