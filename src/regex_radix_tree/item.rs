use std::fmt::Debug;

/// Represent an item added to the regex radix tree
pub trait Item: Debug + Clone + Send + Sync + 'static {
    /// Return the regex used by this item
    ///
    /// Regex should not contains `^` at the start and `$` at the end, those are inserted for you
    /// automatically
    fn regex(&self) -> &str;

    /// Unique identifier for an item
    ///
    /// An item must be included only once in the regex radix tree
    ///
    fn id(&self) -> &str;
}
