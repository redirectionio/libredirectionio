use crate::regex_radix_tree::{Item, Trace, Storage};
use std::fmt::Debug;

pub trait Node<T: Item, S: Storage<T>>: Debug + Send + Sync {
    /// Insert a new item into this node
    fn insert(&mut self, item: T, parent_prefix_size: u32);

    /// Find storage associated to this value
    fn find(&self, value: &str) -> Vec<&S>;

    /// Traces when finding a value
    fn trace(&self, value: &str) -> Trace<T, S>;

    /// Remove an item on this tree
    ///
    /// This method returns true if there is no more data so it can be cleaned up
    fn remove(&mut self, id: &str);

    /// Return regex used by this node
    fn regex(&self) -> &str;

    /// Length of node
    fn len(&self) -> usize;

    /// Is node empty
    fn is_empty(&self) -> bool;

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

    /// Allow to clone object
    fn box_clone(&self) -> Box<dyn Node<T, S>>;
}

impl<T: Item, S: Storage<T>> Clone for Box<dyn Node<T, S>> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}
