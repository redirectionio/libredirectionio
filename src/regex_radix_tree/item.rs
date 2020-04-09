/// Represent an item added to the regex radix tree
pub trait Item: Send + Sync + 'static {
    /// Return the regex used by this item
    ///
    /// Regex should not contains `^` at the start and `$` at the end, those are inserted for you
    /// automatically
    fn node_regex(&self) -> &str;

    fn id(&self) -> &str;
}
