use crate::regex_radix_tree::Item;
use std::fmt::Debug;

pub trait Node<T>: Debug where T: Item + Debug {
    /// Insert a new item into this node
    fn insert(&mut self, item: T);

    /// Find all possible item matching this value
    fn find(&self, value: &str) -> Option<Vec<&T>>;

    /// Return regex used by this node
    fn regex(&self) -> &str;

    fn can_insert_item(&self, prefix: &str, item: &T) -> bool;

    /// Incr level of node by one
    fn incr_level(&mut self);

    /// Cache current regex according to a limit and a level
    ///
    /// This method must return new limit of element cached (passed limit minus number of element cached)
    /// which allow other node to avoid caching extra node
    ///
    /// Implementation must not cache item if limit is equal to 0
    /// Implementation must not cache item if not caching on the current node level
    ///
    /// Level argument allow to build cache on first level of the tree by priority
    /// Implementation must retain at which level this node is build and not do any caching
    /// if we are not on the current level
    fn cache(&mut self, limit: u64, level: u64) -> u64;
}
