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

use serde_json::Map;
use serde_json::Value;

use crate::ConcreteVariantArray;
use crate::ConcreteVariantObject;
use crate::VariantValue;

impl VariantValue for Value {
    type VariantArray = Vec<Value>;
    type VariantObject = Map<String, Value>;

    fn is_array(&self) -> bool {
        self.is_array()
    }

    fn is_object(&self) -> bool {
        self.is_object()
    }

    fn as_array(&self) -> Option<&Self::VariantArray> {
        self.as_array()
    }

    fn as_object(&self) -> Option<&Self::VariantObject> {
        self.as_object()
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
        (**self).len()
    }

    fn get(&self, index: usize) -> Option<&Self::Value> {
        (**self).get(index)
    }

    fn iter(&self) -> impl Iterator<Item = &Self::Value> {
        (**self).iter()
    }
}

impl ConcreteVariantObject for Map<String, Value> {
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
