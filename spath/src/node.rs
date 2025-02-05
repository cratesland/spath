use crate::path::NormalizedPath;
use crate::value::VariantValue;
use std::iter::FusedIterator;
use std::slice::Iter;

/// A list of nodes resulting from a SPath query.
///
/// Each node within the list is a borrowed reference to the node in the original
/// [`VariantValue`] that was queried.
#[derive(Debug, Default, Eq, PartialEq, Clone)]
pub struct NodeList<'a, T: VariantValue>(Vec<&'a T>);

impl<'a, T: VariantValue> NodeList<'a, T> {
    /// Create a new [`NodeList`] from a vector of nodes
    pub fn new(nodes: Vec<&'a T>) -> Self {
        Self(nodes)
    }

    /// Extract *at most* one node from a [`NodeList`]
    ///
    /// This is intended for queries that are expected to optionally yield a single node.
    pub fn at_most_one(&self) -> Result<Option<&'a T>, AtMostOneError> {
        if self.0.len() > 1 {
            Err(AtMostOneError(self.0.len()))
        } else {
            Ok(self.0.first().copied())
        }
    }

    /// Extract *exactly* one node from a [`NodeList`]
    ///
    /// This is intended for queries that are expected to yield exactly one node.
    pub fn exactly_one(&self) -> Result<&'a T, ExactlyOneError> {
        if self.0.len() > 1 {
            Err(ExactlyOneError::MoreThanOne(self.0.len()))
        } else {
            match self.0.first() {
                Some(node) => Ok(*node),
                None => Err(ExactlyOneError::Empty),
            }
        }
    }

    /// Extract all nodes yielded by the query
    ///
    /// This is intended for queries that are expected to yield zero or more nodes.
    pub fn all(self) -> Vec<&'a T> {
        self.0
    }

    /// Get the length of a [`NodeList`]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Check if a [`NodeList`] is empty
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Get an iterator over a [`NodeList`]
    ///
    /// Note that [`NodeList`] also implements [`IntoIterator`].
    pub fn iter(&self) -> Iter<'_, &T> {
        self.0.iter()
    }

    /// Returns the first node in the [`NodeList`], or `None` if it is empty
    pub fn first(&self) -> Option<&'a T> {
        self.0.first().copied()
    }

    /// Returns the last node in the [`NodeList`], or `None` if it is empty
    pub fn last(&self) -> Option<&'a T> {
        self.0.last().copied()
    }

    /// Returns the node at the given index in the [`NodeList`], or `None` if the given index is
    /// out of bounds.
    pub fn get(&self, index: usize) -> Option<&'a T> {
        self.0.get(index).copied()
    }
}

impl<'a, T: VariantValue> IntoIterator for NodeList<'a, T> {
    type Item = &'a T;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

/// A node within a variant value, along with its normalized path location.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct LocatedNode<'a, T: VariantValue> {
    loc: NormalizedPath<'a>,
    node: &'a T,
}

impl<'a, T: VariantValue> LocatedNode<'a, T> {
    /// Create a new located node.
    pub(crate) fn new(loc: NormalizedPath<'a>, node: &'a T) -> Self {
        Self { loc, node }
    }

    /// Get the location of the node as a [`NormalizedPath`].
    pub fn location(&self) -> &NormalizedPath<'a> {
        &self.loc
    }

    /// Take the location of the node as a [`NormalizedPath`].
    pub fn into_location(self) -> NormalizedPath<'a> {
        self.loc
    }

    /// Get the node itself.
    pub fn node(&self) -> &'a T {
        self.node
    }
}

/// A list of [`LocatedNode`] resulting from a SPath query.
#[derive(Debug, Default, Eq, PartialEq, Clone)]
pub struct LocatedNodeList<'a, T: VariantValue>(Vec<LocatedNode<'a, T>>);

impl<'a, T: VariantValue> LocatedNodeList<'a, T> {
    /// Extract *at most* one entry from a [`LocatedNodeList`]
    ///
    /// This is intended for queries that are expected to optionally yield a single node.
    pub fn at_most_one(mut self) -> Result<Option<LocatedNode<'a, T>>, AtMostOneError> {
        if self.0.len() > 1 {
            Err(AtMostOneError(self.0.len()))
        } else {
            Ok(self.0.pop())
        }
    }

    /// Extract *exactly* one entry from a [`LocatedNodeList`]
    ///
    /// This is intended for queries that are expected to yield a single node.
    pub fn exactly_one(mut self) -> Result<LocatedNode<'a, T>, ExactlyOneError> {
        if self.0.is_empty() {
            Err(ExactlyOneError::Empty)
        } else if self.0.len() > 1 {
            Err(ExactlyOneError::MoreThanOne(self.0.len()))
        } else {
            Ok(self.0.pop().unwrap())
        }
    }

    /// Extract all located nodes yielded by the query
    ///
    /// This is intended for queries that are expected to yield zero or more nodes.
    pub fn all(self) -> Vec<LocatedNode<'a, T>> {
        self.0
    }

    /// Get the length of a [`LocatedNodeList`]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Check if a [`LocatedNodeList`] is empty
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Get an iterator over a [`LocatedNodeList`]
    ///
    /// Note that [`LocatedNodeList`] also implements [`IntoIterator`].
    ///
    /// To iterate over just locations, see [`locations`][LocatedNodeList::locations]. To iterate
    /// over just nodes, see [`nodes`][LocatedNodeList::nodes].
    pub fn iter(&self) -> Iter<'_, LocatedNode<'a, T>> {
        self.0.iter()
    }

    /// Get an iterator over the locations of nodes within a [`LocatedNodeList`]
    pub fn locations(&self) -> Locations<'_, T> {
        Locations { inner: self.iter() }
    }

    /// Get an iterator over the nodes within a [`LocatedNodeList`]
    pub fn nodes(&self) -> Nodes<'_, T> {
        Nodes { inner: self.iter() }
    }

    /// Deduplicate a [`LocatedNodeList`] and return the result
    ///
    /// See also, [`dedup_in_place`][LocatedNodeList::dedup_in_place].
    pub fn dedup(mut self) -> Self {
        self.dedup_in_place();
        self
    }

    /// Deduplicate a [`LocatedNodeList`] _in-place_
    ///
    /// See also, [`dedup`][LocatedNodeList::dedup].
    pub fn dedup_in_place(&mut self) {
        self.0.sort();
        self.0.dedup();
    }

    /// Return the first entry in the [`LocatedNodeList`], or `None` if it is empty
    pub fn first(&self) -> Option<&LocatedNode<'a, T>> {
        self.0.first()
    }

    /// Return the last entry in the [`LocatedNodeList`], or `None` if it is empty
    pub fn last(&self) -> Option<&LocatedNode<'a, T>> {
        self.0.last()
    }

    /// Returns the node at the given index in the [`LocatedNodeList`], or `None` if the
    /// given index is out of bounds.
    pub fn get(&self, index: usize) -> Option<&LocatedNode<'a, T>> {
        self.0.get(index)
    }
}

impl<'a, T: VariantValue> IntoIterator for LocatedNodeList<'a, T> {
    type Item = LocatedNode<'a, T>;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

/// An iterator over the locations in a [`LocatedNodeList`]
///
/// Produced by the [`LocatedNodeList::locations`] method.
#[derive(Debug)]
pub struct Locations<'a, T: VariantValue> {
    inner: Iter<'a, LocatedNode<'a, T>>,
}

impl<'a, T: VariantValue> Iterator for Locations<'a, T> {
    type Item = &'a NormalizedPath<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|l| l.location())
    }
}

impl<T: VariantValue> DoubleEndedIterator for Locations<'_, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back().map(|l| l.location())
    }
}

impl<T: VariantValue> ExactSizeIterator for Locations<'_, T> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<T: VariantValue> FusedIterator for Locations<'_, T> {}

/// An iterator over the nodes in a [`LocatedNodeList`]
///
/// Produced by the [`LocatedNodeList::nodes`] method.
#[derive(Debug)]
pub struct Nodes<'a, T: VariantValue> {
    inner: Iter<'a, LocatedNode<'a, T>>,
}

impl<'a, T: VariantValue> Iterator for Nodes<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|l| l.node())
    }
}

impl<T: VariantValue> DoubleEndedIterator for Nodes<'_, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back().map(|l| l.node())
    }
}

impl<T: VariantValue> ExactSizeIterator for Nodes<'_, T> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<T: VariantValue> FusedIterator for Nodes<'_, T> {}

/// Error produced when expecting no more than one node from a query
#[derive(Debug, thiserror::Error)]
#[error("nodelist expected to contain at most one entry, but instead contains {0} entries")]
pub struct AtMostOneError(pub usize);

/// Error produced when expecting exactly one node from a query
#[derive(Debug, thiserror::Error)]
pub enum ExactlyOneError {
    /// The query resulted in an empty [`NodeList`]
    #[error("nodelist expected to contain one entry, but is empty")]
    Empty,
    /// The query resulted in a [`NodeList`] containing more than one node
    #[error("nodelist expected to contain one entry, but instead contains {0} entries")]
    MoreThanOne(usize),
}

impl ExactlyOneError {
    /// Check that it is the `Empty` variant
    pub fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }

    /// Check that it is the `MoreThanOne` variant
    pub fn is_more_than_one(&self) -> bool {
        matches!(self, Self::MoreThanOne(_))
    }

    /// Extract the number of nodes, if it was more than one, or `None` otherwise
    pub fn as_more_than_one(&self) -> Option<usize> {
        match self {
            ExactlyOneError::Empty => None,
            ExactlyOneError::MoreThanOne(u) => Some(*u),
        }
    }
}
