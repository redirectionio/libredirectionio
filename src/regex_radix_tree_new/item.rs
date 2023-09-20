use super::leaf::Leaf;
use super::node::Node;

#[derive(Debug)]
pub enum Item<V> {
    Empty(bool),
    Node(Node<V>),
    Leaf(Leaf<V>),
}

impl<V> Item<V> {
    /// Insert a new item into this node
    pub fn insert(self, regex: &str, id: String, item: V) -> Item<V> {
        match self {
            Item::Empty(ignore_case) => Item::Leaf(Leaf::new(regex, id, item, ignore_case)),
            Item::Node(node) => node.insert(regex, id, item),
            Item::Leaf(leaf) => leaf.insert(regex, id, item),
        }
    }

    /// Find values associated to this haystack
    pub fn find(&self, haystack: &str) -> Vec<&V> {
        match self {
            Item::Empty(_) => Vec::new(),
            Item::Node(node) => node.find(haystack),
            Item::Leaf(leaf) => leaf.find(haystack),
        }
    }

    pub fn get(&self, regex: &str) -> Vec<&V> {
        match self {
            Item::Empty(_) => Vec::new(),
            Item::Node(node) => node.get(regex),
            Item::Leaf(leaf) => leaf.get(regex),
        }
    }

    pub fn get_mut(&mut self, regex: &str) -> Vec<&mut V> {
        match self {
            Item::Empty(_) => Vec::new(),
            Item::Node(node) => node.get_mut(regex),
            Item::Leaf(leaf) => leaf.get_mut(regex),
        }
    }

    /// Remove an item on this tree
    ///
    /// This method returns true if there is no more data so it can be cleaned up
    pub fn remove(self, id: &str) -> (Self, Option<V>) {
        match self {
            Item::Empty(_) => (self, None),
            Item::Node(node) => node.remove(id),
            Item::Leaf(leaf) => leaf.remove(id),
        }
    }

    /// Length of node
    pub fn len(&self) -> usize {
        match self {
            Item::Empty(_) => 0,
            Item::Node(node) => node.len(),
            Item::Leaf(leaf) => leaf.len(),
        }
    }

    pub fn regex(&self) -> &str {
        match self {
            Item::Empty(_) => "",
            Item::Node(node) => node.regex(),
            Item::Leaf(leaf) => leaf.regex(),
        }
    }

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
    pub fn cache(&mut self, left: u64, max_level: u64, current_level: u64) -> u64 {
        if left <= 0 {
            return left;
        }

        if current_level > max_level {
            return left;
        }

        match self {
            Item::Empty(_) => left,
            Item::Node(node) => node.cache(left, max_level, current_level),
            Item::Leaf(leaf) => leaf.cache(left),
        }
    }
}
