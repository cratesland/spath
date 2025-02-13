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

use num_traits::ToPrimitive;

use crate::spec::function::Function;
use crate::spec::function::SPathType;
use crate::spec::function::SPathValue;
use crate::spec::function::ValueType;
use crate::ConcreteVariantArray;
use crate::ConcreteVariantObject;
use crate::Literal;
use crate::VariantValue;

pub fn length<T: VariantValue>() -> Function<T> {
    Function::new(
        "length",
        vec![SPathType::Value],
        SPathType::Value,
        Box::new(move |mut args| {
            assert_eq!(args.len(), 1);

            fn value_len<T: VariantValue>(t: &T) -> Option<usize> {
                if let Some(s) = t.as_str() {
                    Some(s.chars().count())
                } else if let Some(a) = t.as_array() {
                    Some(a.len())
                } else {
                    t.as_object().map(|o| o.len())
                }
            }

            let value = args.pop().unwrap().into_value().unwrap();

            let len = match value {
                ValueType::Value(v) => value_len(&v),
                ValueType::Node(v) => value_len(v),
                ValueType::Nothing => None,
            }
            .and_then(|l| l.to_i64())
            .and_then(|len| T::from_literal(Literal::Int(len)));

            match len {
                Some(v) => SPathValue::Value(v),
                None => SPathValue::Nothing,
            }
        }),
    )
}

pub fn count<T: VariantValue>() -> Function<T> {
    Function::new(
        "count",
        vec![SPathType::Nodes],
        SPathType::Value,
        Box::new(move |mut args| {
            assert_eq!(args.len(), 1);

            let nodes = args.pop().unwrap().into_nodes().unwrap();

            let len = nodes
                .len()
                .to_i64()
                .and_then(|len| T::from_literal(Literal::Int(len)));

            match len {
                Some(v) => SPathValue::Value(v),
                None => SPathValue::Nothing,
            }
        }),
    )
}

pub fn value<T: VariantValue>() -> Function<T> {
    Function::new(
        "value",
        vec![SPathType::Nodes],
        SPathType::Value,
        Box::new(move |mut args| {
            assert_eq!(args.len(), 1);

            let value = args.pop().unwrap().into_nodes().unwrap();
            if value.len() > 1 {
                SPathValue::Nothing
            } else {
                match value.first() {
                    Some(v) => SPathValue::Node(v),
                    None => SPathValue::Nothing,
                }
            }
        }),
    )
}

// 'match' is a keyword in Rust, so we use 'matches' instead.
#[cfg(feature = "regex")]
pub fn matches<T: VariantValue>() -> Function<T> {
    Function::new(
        "match",
        vec![SPathType::Value, SPathType::Value],
        SPathType::Logical,
        Box::new(move |mut args| {
            assert_eq!(args.len(), 2);

            let matcher = args.pop().unwrap().into_value().unwrap();
            let expr = args.pop().unwrap().into_value().unwrap();

            let matches = match (
                matcher.as_value().and_then(|v| v.as_str()),
                expr.as_value().and_then(|v| v.as_str()),
            ) {
                (Some(r), Some(s)) => regex::Regex::new(format!("(?R)^({r})$").as_str())
                    .map(|r| r.is_match(s))
                    .unwrap_or_default(),
                _ => false,
            };

            SPathValue::Logical(matches.into())
        }),
    )
}

#[cfg(feature = "regex")]
pub fn search<T: VariantValue>() -> Function<T> {
    Function::new(
        "search",
        vec![SPathType::Value, SPathType::Value],
        SPathType::Logical,
        Box::new(move |mut args| {
            assert_eq!(args.len(), 2);

            let matcher = args.pop().unwrap().into_value().unwrap();
            let expr = args.pop().unwrap().into_value().unwrap();

            let matches = match (
                matcher.as_value().and_then(|v| v.as_str()),
                expr.as_value().and_then(|v| v.as_str()),
            ) {
                (Some(r), Some(s)) => regex::Regex::new(format!("(?R)({r})").as_str())
                    .map(|r| r.is_match(s))
                    .unwrap_or_default(),
                _ => false,
            };

            SPathValue::Logical(matches.into())
        }),
    )
}
