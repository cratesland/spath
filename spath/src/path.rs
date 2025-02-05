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

//! Types for representing [Normalized Paths] from RFC 9535.
//!
//! [Normalized Paths]: https://datatracker.ietf.org/doc/html/rfc9535#name-normalized-paths

use std::cmp::Ordering;
use std::fmt;
use std::slice::Iter;
use std::slice::SliceIndex;

#[derive(Debug, Default, Eq, PartialEq, Clone, PartialOrd, Ord)]
pub struct NormalizedPath<'a>(Vec<PathElement<'a>>);

impl<'a> NormalizedPath<'a> {
    pub(crate) fn push<T: Into<PathElement<'a>>>(&mut self, elem: T) {
        self.0.push(elem.into())
    }

    pub(crate) fn clone_and_push<T: Into<PathElement<'a>>>(&self, elem: T) -> Self {
        let mut new_path = self.clone();
        new_path.push(elem.into());
        new_path
    }

    /// Check if the [`NormalizedPath`] is empty
    ///
    /// An empty normalized path represents the location of the root node of the object,
    /// i.e., `$`.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Get the length of the [`NormalizedPath`]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Get an iterator over the [`PathElement`]s of the [`NormalizedPath`]
    ///
    /// Note that [`NormalizedPath`] also implements [`IntoIterator`]
    pub fn iter(&self) -> Iter<'_, PathElement<'a>> {
        self.0.iter()
    }

    /// Get the [`PathElement`] at `index`, or `None` if the index is out of bounds
    pub fn get<I>(&self, index: I) -> Option<&I::Output>
    where
        I: SliceIndex<[PathElement<'a>]>,
    {
        self.0.get(index)
    }

    /// Get the first [`PathElement`], or `None` if the path is empty
    pub fn first(&self) -> Option<&PathElement<'a>> {
        self.0.first()
    }

    /// Get the last [`PathElement`], or `None` if the path is empty
    pub fn last(&self) -> Option<&PathElement<'a>> {
        self.0.last()
    }
}

impl<'a> IntoIterator for NormalizedPath<'a> {
    type Item = PathElement<'a>;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl fmt::Display for NormalizedPath<'_> {
    /// Format the [`NormalizedPath`] as a SPath string using the canonical bracket notation
    /// as per [RFC 9535][norm-paths]
    ///
    /// [norm-paths]: https://datatracker.ietf.org/doc/html/rfc9535#name-normalized-paths
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "$")?;
        for elem in &self.0 {
            match elem {
                PathElement::Name(name) => write!(f, "['{name}']")?,
                PathElement::Index(index) => write!(f, "[{index}]")?,
            }
        }
        Ok(())
    }
}

/// An element within a [`NormalizedPath`]
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum PathElement<'a> {
    /// A key within an object
    Name(&'a str),
    /// An index of an array
    Index(usize),
}

impl PathElement<'_> {
    /// Get the underlying name if the [`PathElement`] is `Name`, or `None` otherwise
    pub fn as_name(&self) -> Option<&str> {
        match self {
            PathElement::Name(n) => Some(n),
            PathElement::Index(_) => None,
        }
    }

    /// Get the underlying index if the [`PathElement`] is `Index`, or `None` otherwise
    pub fn as_index(&self) -> Option<usize> {
        match self {
            PathElement::Name(_) => None,
            PathElement::Index(i) => Some(*i),
        }
    }

    /// Test if the [`PathElement`] is `Name`
    pub fn is_name(&self) -> bool {
        self.as_name().is_some()
    }

    /// Test if the [`PathElement`] is `Index`
    pub fn is_index(&self) -> bool {
        self.as_index().is_some()
    }
}

impl PartialOrd for PathElement<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PathElement<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (PathElement::Name(a), PathElement::Name(b)) => a.cmp(b),
            (PathElement::Index(a), PathElement::Index(b)) => a.cmp(b),
            (PathElement::Name(_), PathElement::Index(_)) => Ordering::Greater,
            (PathElement::Index(_), PathElement::Name(_)) => Ordering::Less,
        }
    }
}

impl PartialEq<str> for PathElement<'_> {
    fn eq(&self, other: &str) -> bool {
        match self {
            PathElement::Name(s) => s.eq(&other),
            PathElement::Index(_) => false,
        }
    }
}

impl PartialEq<&str> for PathElement<'_> {
    fn eq(&self, other: &&str) -> bool {
        match self {
            PathElement::Name(s) => s.eq(other),
            PathElement::Index(_) => false,
        }
    }
}

impl PartialEq<usize> for PathElement<'_> {
    fn eq(&self, other: &usize) -> bool {
        match self {
            PathElement::Name(_) => false,
            PathElement::Index(i) => i.eq(other),
        }
    }
}

impl fmt::Display for PathElement<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PathElement::Name(n) => {
                // https://datatracker.ietf.org/doc/html/rfc9535#name-normalized-paths
                for c in n.chars() {
                    match c {
                        '\u{0008}' => write!(f, r#"\b"#)?, // b BS backspace
                        '\u{000C}' => write!(f, r#"\f"#)?, // f FF form feed
                        '\u{000A}' => write!(f, r#"\n"#)?, // n LF line feed
                        '\u{000D}' => write!(f, r#"\r"#)?, // r CR carriage return
                        '\u{0009}' => write!(f, r#"\t"#)?, // t HT horizontal tab
                        '\u{0027}' => write!(f, r#"\'"#)?, // ' apostrophe
                        '\u{005C}' => write!(f, r#"\"#)?,  // \ backslash (reverse solidus)
                        ('\x00'..='\x07') | '\x0b' | '\x0e' | '\x0f' => {
                            // "00"-"07", "0b", "0e"-"0f"
                            write!(f, "\\u000{:x}", c as i32)?
                        }
                        _ => write!(f, "{c}")?,
                    }
                }
                Ok(())
            }
            PathElement::Index(i) => write!(f, "{i}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;

    use super::PathElement;

    #[test]
    fn test_normalized_element() {
        // simple name
        assert_snapshot!(PathElement::Name("foo"), @"foo");
        // index
        assert_snapshot!(PathElement::Index(1), @"1");
        // escape_apostrophes
        assert_snapshot!(PathElement::Name("'hi'"), @r#"\'hi\'"#);
        // escapes
        assert_snapshot!(PathElement::Name(r#"'\b\f\n\r\t\\'"#), @r#"\'\b\f\n\r\t\\\'"#);
        // escape_vertical_unicode
        assert_snapshot!(PathElement::Name("\u{000B}"), @r#"\u000b"#);
        // escape_unicode_null
        assert_snapshot!(PathElement::Name("\u{0000}"), @r#"\u0000"#);
        // escape_unicode_runes
        assert_snapshot!(PathElement::Name(
            "\u{0001}\u{0002}\u{0003}\u{0004}\u{0005}\u{0006}\u{0007}\u{000e}\u{000F}"
        ), @r#"\u0001\u0002\u0003\u0004\u0005\u0006\u0007\u000e\u000f"#);
    }
}
