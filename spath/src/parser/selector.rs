#[derive(Debug, Clone)]
pub enum Selector {
    /// §2.3.2 Wildcard Selector.
    Asterisk,
    /// §2.3.1 Name Selector.
    Identifier {
        /// The name of the selector.
        name: String,
    },
    /// §2.3.3 Index Selector.
    Index {
        /// The index of the selector.
        index: i64,
    },
    /// §2.3.4 Array Slice Selector.
    Slice {
        /// The start index of the slice, inclusive. Default to 0.
        start: Option<i64>,
        /// The end index of the slice, exclusive. Default to the length of the array.
        end: Option<i64>,
        /// The step to iterate the slice. Default to 1.
        step: Option<i64>,
    },
}
