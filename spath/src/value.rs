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

pub trait VariantValue: Clone {
    type VariantArray: ConcreteVariantArray<Value = Self>;
    type VariantObject: ConcreteVariantObject<Value = Self>;
    fn is_array(&self) -> bool;
    fn is_object(&self) -> bool;
    fn as_array(&self) -> Option<&Self::VariantArray>;
    fn as_object(&self) -> Option<&Self::VariantObject>;

    fn make_array(iter: impl IntoIterator<Item = Self>) -> Self;
}

pub trait ConcreteVariantArray {
    type Value: VariantValue<VariantArray = Self>;
    fn is_empty(&self) -> bool;
    fn len(&self) -> usize;
    fn get(&self, index: usize) -> Option<&Self::Value>;
    fn iter(&self) -> impl Iterator<Item = &Self::Value>;
}

pub trait ConcreteVariantObject {
    type Value: VariantValue<VariantObject = Self>;
    fn is_empty(&self) -> bool;
    fn len(&self) -> usize;
    fn get(&self, key: &str) -> Option<&Self::Value>;
    fn values(&self) -> impl Iterator<Item = &Self::Value>;
}
