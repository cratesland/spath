use crate::{ConcreteArrayRef, ConcreteObjectRef, VariantValue};
use serde_json::{Map, Value};

impl VariantValue for Value {
    type ArrayRef<'a> = &'a Vec<Value>;
    type ObjectRef<'a> = &'a Map<String, Value>;

    fn is_array(&self) -> bool {
        self.is_array()
    }

    fn is_object(&self) -> bool {
        self.is_object()
    }

    fn as_array(&self) -> Option<Self::ArrayRef> {
        self.as_array()
    }

    fn as_object(&self) -> Option<Self::ObjectRef> {
        self.as_object()
    }

    fn make_array(iter: impl IntoIterator<Item = Self>) -> Self {
        Value::Array(iter.into_iter().collect())
    }
}

impl<'a> ConcreteArrayRef<'a> for &'a Vec<Value> {
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

impl<'a> ConcreteObjectRef<'a> for &'a Map<String, Value> {
    type Value = Value;

    fn len(&self) -> usize {
        (**self).len()
    }

    fn get(&self, key: &str) -> Option<&Self::Value> {
        (**self).get(key)
    }

    fn values(&self) -> impl Iterator<Item = &Self::Value> {
        (**self).values()
    }
}
