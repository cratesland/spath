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

use toml::Table;
use toml::Value;

use crate::ConcreteVariantArray;
use crate::ConcreteVariantObject;
use crate::VariantValue;

impl VariantValue for Value {
    type VariantArray = Vec<Value>;
    type VariantObject = Table;

    fn is_array(&self) -> bool {
        self.is_array()
    }

    fn is_object(&self) -> bool {
        self.is_table()
    }

    fn as_array(&self) -> Option<&Self::VariantArray> {
        self.as_array()
    }

    fn as_object(&self) -> Option<&Self::VariantObject> {
        self.as_table()
    }

    fn make_array(iter: impl IntoIterator<Item = Self>) -> Self {
        Value::Array(iter.into_iter().collect())
    }
}

impl ConcreteVariantArray for Vec<Value> {
    type Value = Value;

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn get(&self, index: usize) -> Option<&Self::Value> {
        (**self).get(index)
    }

    fn iter(&self) -> impl Iterator<Item = &Self::Value> {
        (**self).iter()
    }
}

impl ConcreteVariantObject for Table {
    type Value = Value;

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn get(&self, key: &str) -> Option<&Self::Value> {
        self.get(key)
    }

    fn values(&self) -> impl Iterator<Item = &Self::Value> {
        self.values()
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_compact_json_snapshot;

    use super::*;
    use crate::manifest_dir;
    use crate::SPath;

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
}
