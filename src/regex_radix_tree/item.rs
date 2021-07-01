use std::fmt::Debug;

/// Represent an item added to the regex radix tree
pub trait NodeItem: Debug + Clone + Send + Sync + 'static {
    /// Return the regex used by this item
    ///
    /// Regex should not contains `^` at the start and `$` at the end, those are inserted for you
    /// automatically
    fn regex(&self) -> &str;

    /// Should the regex be case insensitive
    fn case_insensitive(&self) -> bool;
}
