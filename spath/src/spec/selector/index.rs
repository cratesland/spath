//! Index selectors in SPath.

use crate::spec::integer::Integer;
use crate::spec::query::Queryable;
use crate::{ConcreteVariantArray, LocatedNode, NormalizedPath, VariantValue};
use std::fmt;

/// For selecting array elements by their index.
///
/// Can use negative indices to index from the end of an array.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Index(pub Integer);

impl fmt::Display for Index {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Queryable for Index {
    fn query<'b, T: VariantValue>(&self, current: &'b T, _root: &'b T) -> Vec<&'b T> {
        if let Some(list) = current.as_array() {
            if self.0 < 0 {
                let abs = self.0.abs();
                usize::try_from(abs)
                    .ok()
                    .and_then(|i| list.len().checked_sub(i))
                    .and_then(|i| list.get(i))
                    .map(|v| vec![v])
                    .unwrap_or_default()
            } else {
                usize::try_from(self.0)
                    .ok()
                    .and_then(|i| list.get(i))
                    .map(|v| vec![v])
                    .unwrap_or_default()
            }
        } else {
            vec![]
        }
    }

    fn query_located<'b, T: VariantValue>(
        &self,
        current: &'b T,
        _root: &'b T,
        mut parent: NormalizedPath<'b>,
    ) -> Vec<LocatedNode<'b, T>> {
        if let Some((index, node)) = current.as_array().and_then(|list| {
            if self.0 < 0 {
                let abs = self.0.abs();
                usize::try_from(abs)
                    .ok()
                    .and_then(|i| list.len().checked_sub(i))
                    .and_then(|i| list.get(i).map(|v| (i, v)))
            } else {
                usize::try_from(self.0)
                    .ok()
                    .and_then(|i| list.get(i).map(|v| (i, v)))
            }
        }) {
            parent.push(index);
            vec![LocatedNode::new(parent, node)]
        } else {
            vec![]
        }
    }
}
