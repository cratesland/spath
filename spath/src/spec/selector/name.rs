//! Name selectors for selecting object keys in SPath.

use crate::spec::query::Queryable;
use crate::{ConcreteVariantObject, LocatedNode, NormalizedPath, VariantValue};
use std::fmt;

/// Select a single variant object key.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Name(pub String);

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "'{name}'", name = self.0)
    }
}

impl Queryable for Name {
    fn query<'b, T: VariantValue>(&self, current: &'b T, _root: &'b T) -> Vec<&'b T> {
        current
            .as_object()
            .and_then(|o| o.get(&self.0))
            .map(|v| vec![v])
            .unwrap_or_default()
    }

    fn query_located<'b, T: VariantValue>(
        &self,
        current: &'b T,
        _root: &'b T,
        mut parent: NormalizedPath<'b>,
    ) -> Vec<LocatedNode<'b, T>> {
        current
            .as_object()
            .and_then(|o| o.get_key_value(&self.0))
            .map(|(k, v)| {
                parent.push(k);
                vec![LocatedNode::new(parent, v)]
            })
            .unwrap_or_default()
    }
}

impl From<&str> for Name {
    fn from(s: &str) -> Self {
        Self(s.to_owned())
    }
}
