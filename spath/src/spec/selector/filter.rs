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

//! Types representing filter selectors in SPath.

use std::fmt;

use super::index::Index;
use super::name::Name;
use super::Selector;
use crate::node::LocatedNode;
use crate::path::NormalizedPath;
use crate::spec::functions::FunctionExpr;
use crate::spec::functions::FunctionRegistry;
use crate::spec::functions::SPathValue;
use crate::spec::query::Query;
use crate::spec::query::QueryKind;
use crate::spec::query::Queryable;
use crate::spec::segment::QuerySegment;
use crate::spec::segment::Segment;
use crate::ConcreteVariantArray;
use crate::ConcreteVariantObject;
use crate::Literal;
use crate::VariantValue;

mod sealed {
    use super::BasicExpr;
    use super::ComparisonExpr;
    use super::ExistExpr;
    use super::LogicalAndExpr;
    use super::LogicalOrExpr;

    pub trait Sealed {}
    impl Sealed for LogicalOrExpr {}
    impl Sealed for LogicalAndExpr {}
    impl Sealed for BasicExpr {}
    impl Sealed for ExistExpr {}
    impl Sealed for ComparisonExpr {}
}

/// Trait for testing a filter type.
pub trait TestFilter: sealed::Sealed {
    /// Test self using the current and root nodes.
    fn test_filter<'b, T: VariantValue, R: FunctionRegistry<Value = T>>(
        &self,
        current: &'b T,
        root: &'b T,
        registry: &R,
    ) -> bool;
}

/// The main filter type for SPath.
#[derive(Debug, Clone)]
pub struct Filter(pub LogicalOrExpr);

impl fmt::Display for Filter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{expr}", expr = self.0)
    }
}

impl Queryable for Filter {
    fn query<'b, T: VariantValue, R: FunctionRegistry<Value = T>>(
        &self,
        current: &'b T,
        root: &'b T,
        registry: &R,
    ) -> Vec<&'b T> {
        if let Some(list) = current.as_array() {
            list.iter()
                .filter(|v| self.0.test_filter(*v, root, registry))
                .collect()
        } else if let Some(obj) = current.as_object() {
            obj.iter()
                .map(|(_, v)| v)
                .filter(|v| self.0.test_filter(*v, root, registry))
                .collect()
        } else {
            vec![]
        }
    }

    fn query_located<'b, T: VariantValue, R: FunctionRegistry<Value = T>>(
        &self,
        current: &'b T,
        root: &'b T,
        registry: &R,
        parent: NormalizedPath<'b>,
    ) -> Vec<LocatedNode<'b, T>> {
        if let Some(list) = current.as_array() {
            list.iter()
                .enumerate()
                .filter(|(_, v)| self.0.test_filter(*v, root, registry))
                .map(|(i, v)| LocatedNode::new(parent.clone_and_push(i), v))
                .collect()
        } else if let Some(obj) = current.as_object() {
            obj.iter()
                .filter(|(_, v)| self.0.test_filter(*v, root, registry))
                .map(|(k, v)| LocatedNode::new(parent.clone_and_push(k), v))
                .collect()
        } else {
            vec![]
        }
    }
}

/// The top level boolean expression type.
///
/// This is also `logical-expression` in RFC 9535, but the naming was chosen to
/// make it more clear that it represents the logical OR, and to not have an extra wrapping type.
#[derive(Debug, Clone)]
pub struct LogicalOrExpr(pub Vec<LogicalAndExpr>);

impl fmt::Display for LogicalOrExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, expr) in self.0.iter().enumerate() {
            write!(
                f,
                "{expr}{logic}",
                logic = if i == self.0.len() - 1 { "" } else { " || " }
            )?;
        }
        Ok(())
    }
}

impl TestFilter for LogicalOrExpr {
    fn test_filter<'b, T: VariantValue, R: FunctionRegistry<Value = T>>(
        &self,
        current: &'b T,
        root: &'b T,
        registry: &R,
    ) -> bool {
        self.0
            .iter()
            .any(|expr| expr.test_filter(current, root, registry))
    }
}

/// A logical AND expression.
#[derive(Debug, Clone)]
pub struct LogicalAndExpr(pub Vec<BasicExpr>);

impl fmt::Display for LogicalAndExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, expr) in self.0.iter().enumerate() {
            write!(
                f,
                "{expr}{logic}",
                logic = if i == self.0.len() - 1 { "" } else { " && " }
            )?;
        }
        Ok(())
    }
}

impl TestFilter for LogicalAndExpr {
    fn test_filter<'b, T: VariantValue, R: FunctionRegistry<Value = T>>(
        &self,
        current: &'b T,
        root: &'b T,
        registry: &R,
    ) -> bool {
        self.0
            .iter()
            .all(|expr| expr.test_filter(current, root, registry))
    }
}

/// The basic for m of expression in a filter.
#[derive(Debug, Clone)]
pub enum BasicExpr {
    /// An expression wrapped in parentheses
    Paren(LogicalOrExpr),
    /// A parenthesized expression preceded with a `!`
    ParenNot(LogicalOrExpr),
    /// A relationship expression which compares two JSON values
    Relation(ComparisonExpr),
    /// An existence expression
    Exist(ExistExpr),
    /// The inverse of an existence expression, i.e., preceded by `!`
    NotExist(ExistExpr),
}

impl fmt::Display for BasicExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BasicExpr::Paren(expr) => write!(f, "({expr})"),
            BasicExpr::ParenNot(expr) => write!(f, "!({expr})"),
            BasicExpr::Relation(rel) => write!(f, "{rel}"),
            BasicExpr::Exist(exist) => write!(f, "{exist}"),
            BasicExpr::NotExist(exist) => write!(f, "!{exist}"),
        }
    }
}

impl BasicExpr {
    /// Optionally express as a relation expression
    pub fn as_relation(&self) -> Option<&ComparisonExpr> {
        match self {
            BasicExpr::Relation(cx) => Some(cx),
            _ => None,
        }
    }
}

impl TestFilter for BasicExpr {
    fn test_filter<'b, T: VariantValue, R: FunctionRegistry<Value = T>>(
        &self,
        current: &'b T,
        root: &'b T,
        registry: &R,
    ) -> bool {
        match self {
            BasicExpr::Paren(expr) => expr.test_filter(current, root, registry),
            BasicExpr::ParenNot(expr) => !expr.test_filter(current, root, registry),
            BasicExpr::Relation(expr) => expr.test_filter(current, root, registry),
            BasicExpr::Exist(expr) => expr.test_filter(current, root, registry),
            BasicExpr::NotExist(expr) => !expr.test_filter(current, root, registry),
        }
    }
}

/// Existence expression.
#[derive(Debug, Clone)]
pub struct ExistExpr(pub Query);

impl fmt::Display for ExistExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{query}", query = self.0)
    }
}

impl TestFilter for ExistExpr {
    fn test_filter<'b, T: VariantValue, R: FunctionRegistry<Value = T>>(
        &self,
        current: &'b T,
        root: &'b T,
        registry: &R,
    ) -> bool {
        !self.0.query(current, root, registry).is_empty()
    }
}

/// A comparison expression comparing two JSON values
#[derive(Debug, Clone)]
pub struct ComparisonExpr {
    /// The JSON value on the left of the comparison
    pub left: Comparable,
    /// The operator of comparison
    pub op: ComparisonOperator,
    /// The JSON value on the right of the comparison
    pub right: Comparable,
}

impl fmt::Display for ComparisonExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{left}{op}{right}",
            left = self.left,
            op = self.op,
            right = self.right
        )
    }
}

impl TestFilter for ComparisonExpr {
    fn test_filter<'b, T: VariantValue, R: FunctionRegistry<Value = T>>(
        &self,
        current: &'b T,
        root: &'b T,
        registry: &R,
    ) -> bool {
        let left = self.left.as_value(current, root, registry);
        let right = self.right.as_value(current, root, registry);
        match self.op {
            ComparisonOperator::EqualTo => check_equal_to(&left, &right),
            ComparisonOperator::NotEqualTo => !check_equal_to(&left, &right),
            ComparisonOperator::LessThan => check_less_than(&left, &right),
            ComparisonOperator::GreaterThan => {
                !check_less_than(&left, &right) && !check_equal_to(&left, &right)
            }
            ComparisonOperator::LessThanEqualTo => {
                check_less_than(&left, &right) || check_equal_to(&left, &right)
            }
            ComparisonOperator::GreaterThanEqualTo => !check_less_than(&left, &right),
        }
    }
}

fn check_equal_to<T: VariantValue>(left: &SPathValue<T>, right: &SPathValue<T>) -> bool {
    let (left, right) = match (left, right) {
        (SPathValue::Node(v1), SPathValue::Node(v2)) => (*v1, *v2),
        (SPathValue::Node(v1), SPathValue::Value(v2)) => (*v1, v2),
        (SPathValue::Value(v1), SPathValue::Node(v2)) => (v1, *v2),
        (SPathValue::Value(v1), SPathValue::Value(v2)) => (v1, v2),
        (SPathValue::Nothing, SPathValue::Nothing) => return true,
        _ => return false,
    };

    left.is_equal_to(right)
}

fn check_less_than<T: VariantValue>(left: &SPathValue<T>, right: &SPathValue<T>) -> bool {
    let (left, right) = match (left, right) {
        (SPathValue::Node(v1), SPathValue::Node(v2)) => (*v1, *v2),
        (SPathValue::Node(v1), SPathValue::Value(v2)) => (*v1, v2),
        (SPathValue::Value(v1), SPathValue::Node(v2)) => (v1, *v2),
        (SPathValue::Value(v1), SPathValue::Value(v2)) => (v1, v2),
        _ => return false,
    };

    left.is_less_than(right)
}

/// The comparison operator
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ComparisonOperator {
    /// `==`
    EqualTo,
    /// `!=`
    NotEqualTo,
    /// `<`
    LessThan,
    /// `>`
    GreaterThan,
    /// `<=`
    LessThanEqualTo,
    /// `>=`
    GreaterThanEqualTo,
}

impl fmt::Display for ComparisonOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ComparisonOperator::EqualTo => write!(f, "=="),
            ComparisonOperator::NotEqualTo => write!(f, "!="),
            ComparisonOperator::LessThan => write!(f, "<"),
            ComparisonOperator::GreaterThan => write!(f, ">"),
            ComparisonOperator::LessThanEqualTo => write!(f, "<="),
            ComparisonOperator::GreaterThanEqualTo => write!(f, ">="),
        }
    }
}

/// A type that is comparable
#[derive(Debug, Clone)]
pub enum Comparable {
    /// A literal variant value, excluding objects and arrays.
    Literal(Literal),
    /// A singular query.
    ///
    /// This will only produce a single node, i.e., a variant value, or nothing
    SingularQuery(SingularQuery),
    /// A function expression that can only produce a `ValueType`
    FunctionExpr(FunctionExpr),
}

impl fmt::Display for Comparable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Comparable::Literal(lit) => write!(f, "{lit}"),
            Comparable::SingularQuery(path) => write!(f, "{path}"),
            Comparable::FunctionExpr(expr) => write!(f, "{expr}"),
        }
    }
}

impl Comparable {
    /// Convert the comparable variable to a variant value.
    pub fn as_value<'a, 'b: 'a, T: VariantValue, R: FunctionRegistry<Value = T>>(
        &'a self,
        current: &'b T,
        root: &'b T,
        registry: &R,
    ) -> SPathValue<'a, T> {
        match self {
            Comparable::Literal(lit) => match T::from_literal(lit.clone()) {
                Some(v) => SPathValue::Value(v),
                None => SPathValue::Nothing,
            },
            Comparable::SingularQuery(sp) => match sp.eval_query(current, root) {
                Some(v) => SPathValue::Node(v),
                None => SPathValue::Nothing,
            },
            Comparable::FunctionExpr(expr) => expr.evaluate(current, root, registry),
        }
    }
}

/// A segment in a singular query
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SingularQuerySegment {
    /// A single name segment
    Name(Name),
    /// A single index segment
    Index(Index),
}

impl fmt::Display for SingularQuerySegment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SingularQuerySegment::Name(name) => write!(f, "{name}"),
            SingularQuerySegment::Index(index) => write!(f, "{index}"),
        }
    }
}

impl TryFrom<QuerySegment> for SingularQuerySegment {
    type Error = NonSingularQueryError;

    fn try_from(segment: QuerySegment) -> Result<Self, Self::Error> {
        if segment.is_descendent() {
            return Err(NonSingularQueryError::Descendant);
        }
        match segment.segment {
            Segment::LongHand(mut selectors) => {
                if selectors.len() > 1 {
                    Err(NonSingularQueryError::TooManySelectors)
                } else if let Some(sel) = selectors.pop() {
                    sel.try_into()
                } else {
                    Err(NonSingularQueryError::NoSelectors)
                }
            }
            Segment::DotName(name) => Ok(Self::Name(Name::new(name))),
            Segment::Wildcard => Err(NonSingularQueryError::Wildcard),
        }
    }
}

impl TryFrom<Selector> for SingularQuerySegment {
    type Error = NonSingularQueryError;

    fn try_from(selector: Selector) -> Result<Self, Self::Error> {
        match selector {
            Selector::Name(n) => Ok(Self::Name(n)),
            Selector::Wildcard => Err(NonSingularQueryError::Wildcard),
            Selector::Index(i) => Ok(Self::Index(i)),
            Selector::ArraySlice(_) => Err(NonSingularQueryError::Slice),
            Selector::Filter(_) => Err(NonSingularQueryError::Filter),
        }
    }
}

/// Represents a singular query in JSONPath
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SingularQuery {
    /// The kind of singular query, relative or absolute
    pub kind: SingularQueryKind,
    /// The segments making up the query
    pub segments: Vec<SingularQuerySegment>,
}

impl SingularQuery {
    /// Evaluate the singular query
    pub fn eval_query<'b, T: VariantValue>(&self, current: &'b T, root: &'b T) -> Option<&'b T> {
        let mut target = match self.kind {
            SingularQueryKind::Absolute => root,
            SingularQueryKind::Relative => current,
        };
        for segment in &self.segments {
            match segment {
                SingularQuerySegment::Name(name) => {
                    if let Some(t) = target.as_object().and_then(|o| o.get(name.as_str())) {
                        target = t;
                    } else {
                        return None;
                    }
                }
                SingularQuerySegment::Index(i) => {
                    if let Some(t) = target
                        .as_array()
                        .and_then(|l| usize::try_from(i.index()).ok().and_then(|i| l.get(i)))
                    {
                        target = t;
                    } else {
                        return None;
                    }
                }
            }
        }
        Some(target)
    }
}

impl TryFrom<Query> for SingularQuery {
    type Error = NonSingularQueryError;

    fn try_from(query: Query) -> Result<Self, Self::Error> {
        let kind = SingularQueryKind::from(query.kind);
        let segments = query
            .segments
            .into_iter()
            .map(TryFrom::try_from)
            .collect::<Result<Vec<SingularQuerySegment>, Self::Error>>()?;
        Ok(Self { kind, segments })
    }
}

impl fmt::Display for SingularQuery {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            SingularQueryKind::Absolute => write!(f, "$")?,
            SingularQueryKind::Relative => write!(f, "@")?,
        }
        for s in &self.segments {
            write!(f, "[{s}]")?;
        }
        Ok(())
    }
}

/// The kind of singular query
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SingularQueryKind {
    /// Referencing the root node, i.e., `$`
    Absolute,
    /// Referencing the current node, i.e., `@`
    Relative,
}

impl From<QueryKind> for SingularQueryKind {
    fn from(qk: QueryKind) -> Self {
        match qk {
            QueryKind::Root => Self::Absolute,
            QueryKind::Current => Self::Relative,
        }
    }
}

/// Error when parsing a singular query
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum NonSingularQueryError {
    /// Descendant segment
    #[error("descendant segments are not singular")]
    Descendant,
    /// Long hand segment with too many internal selectors
    #[error("long hand segment contained more than one selector")]
    TooManySelectors,
    /// Long hand segment with no selectors
    #[error("long hand segment contained no selectors")]
    NoSelectors,
    /// A wildcard segment
    #[error("wildcard segments are not singular")]
    Wildcard,
    /// A slice segment
    #[error("slice segments are not singular")]
    Slice,
    /// A filter segment
    #[error("filter segments are not singular")]
    Filter,
}
