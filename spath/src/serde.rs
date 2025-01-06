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

use core::fmt;

use num_traits::ToPrimitive;
use serde::de;
use serde::de::Visitor;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;

use crate::Number;
use crate::Value;

impl Serialize for Number {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            Number::U64(u) => serializer.serialize_u64(u),
            Number::I64(i) => serializer.serialize_i64(i),
            Number::F64(f) => serializer.serialize_f64(f.0),
        }
    }
}

impl<'de> Deserialize<'de> for Number {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Number, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NumberVisitor;

        impl Visitor<'_> for NumberVisitor {
            type Value = Number;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a variant number")
            }

            fn visit_i64<E>(self, value: i64) -> Result<Number, E> {
                Ok(value.into())
            }

            fn visit_i128<E>(self, value: i128) -> Result<Number, E>
            where
                E: de::Error,
            {
                let n = value
                    .to_i64()
                    .ok_or_else(|| de::Error::custom("variant number out of range"))?;
                Ok(n.into())
            }

            fn visit_u64<E>(self, value: u64) -> Result<Number, E> {
                Ok(value.into())
            }

            fn visit_u128<E>(self, value: u128) -> Result<Number, E>
            where
                E: de::Error,
            {
                let n = value
                    .to_u64()
                    .ok_or_else(|| de::Error::custom("variant number out of range"))?;
                Ok(n.into())
            }

            fn visit_f64<E>(self, value: f64) -> Result<Number, E>
            where
                E: de::Error,
            {
                Ok(value.into())
            }
        }

        deserializer.deserialize_any(NumberVisitor)
    }
}

impl Serialize for Value {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Value::Null => serializer.serialize_unit(),
            Value::Bool(b) => serializer.serialize_bool(*b),
            Value::Number(n) => n.serialize(serializer),
            Value::String(s) => serializer.serialize_str(s),
            Value::Timestamp(ts) => ts.serialize(serializer),
            Value::Interval(sd) => sd.serialize(serializer),
            Value::Binary(bs) => bs.serialize(serializer),
            Value::Array(v) => v.serialize(serializer),
            Value::Object(m) => m.serialize(serializer),
        }
    }
}
