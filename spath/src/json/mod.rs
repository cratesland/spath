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

use serde_json::Value as JsonValue;

use crate::Number;
use crate::Value;

#[cfg(test)]
mod tests;

impl From<JsonValue> for Value {
    fn from(value: JsonValue) -> Self {
        match value {
            JsonValue::Null => Value::Null,
            JsonValue::Bool(b) => Value::Bool(b),
            JsonValue::Number(n) => {
                if let Some(i) = n.as_i64() {
                    Value::Number(Number::I64(i))
                } else if let Some(u) = n.as_u64() {
                    Value::Number(Number::U64(u))
                } else {
                    // always possible
                    let n = n.as_f64().unwrap();
                    Value::Number(Number::F64(n.into()))
                }
            }
            JsonValue::String(s) => Value::String(s),
            JsonValue::Array(a) => Value::Array(a.into_iter().map(Value::from).collect()),
            JsonValue::Object(o) => {
                Value::Object(o.into_iter().map(|(k, v)| (k, Value::from(v))).collect())
            }
        }
    }
}

impl From<Value> for JsonValue {
    fn from(value: Value) -> Self {
        match value {
            Value::Null => JsonValue::Null,
            Value::Bool(b) => JsonValue::Bool(b),
            Value::Number(n) => match n {
                Number::I64(i) => JsonValue::Number(i.into()),
                Number::U64(u) => JsonValue::Number(u.into()),
                Number::F64(f) => match serde_json::Number::from_f64(f.0) {
                    None => JsonValue::Null,
                    Some(n) => JsonValue::Number(n),
                },
            },
            Value::String(s) => JsonValue::String(s),
            Value::Timestamp(ts) => JsonValue::String(format!("{ts}")),
            Value::Interval(sd) => JsonValue::String(format!("{sd}")),
            Value::Binary(b) => JsonValue::String(String::from_utf8_lossy(&b).to_string()),
            Value::Array(a) => JsonValue::Array(a.into_iter().map(JsonValue::from).collect()),
            Value::Object(o) => JsonValue::Object(
                o.into_iter()
                    .map(|(k, v)| (k, JsonValue::from(v)))
                    .collect(),
            ),
        }
    }
}
