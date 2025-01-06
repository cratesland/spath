use crate::{ConcreteVariantArray, ConcreteVariantObject, VariantValue};
use serde_json::{Map, Value};

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
