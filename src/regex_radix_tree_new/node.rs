use super::item::Item;
use super::leaf::Leaf;
use super::prefix::{common_prefix_char_size, get_prefix_with_char_size};
use super::regex::LazyRegex;

#[derive(Debug)]
pub struct Node<V> {
    pub(crate) regex: LazyRegex,
    pub(crate) children: Vec<Item<V>>,
}

impl<V> Node<V> {
    /// Insert a new item into this node
    pub fn insert(mut self, regex: &str, id: String, item: V) -> Item<V> {
        let mut max_prefix_size = self.regex.original.len() as u32;
        let prefix_size = common_prefix_char_size(regex, self.regex.original.as_str());

        if prefix_size < max_prefix_size {
            let prefix = get_prefix_with_char_size(self.regex.original.as_str(), prefix_size);

            let left = Item::Leaf(Leaf::new(regex, id, item, self.regex.ignore_case));

            return Item::Node(Node {
                regex: LazyRegex::new_node(prefix, self.regex.ignore_case),
                children: vec![left, Item::Node(self)],
            });
        }

        let mut max_prefix_item = None;

        for i in 0..self.children.len() {
            let prefix_size = common_prefix_char_size(regex, self.children[i].regex());

            if prefix_size > max_prefix_size {
                max_prefix_size = prefix_size;
                max_prefix_item = Some(i);
            }
        }

        match max_prefix_item {
            Some(child_index) => {
                let mut children = self.children.remove(child_index);
                children = children.insert(regex, id, item);
                self.children.push(children);
            }
            None => {
                self.children.push(Item::Leaf(Leaf::new(regex, id, item, self.regex.ignore_case)));
            }
        }

        Item::Node(self)
    }

    /// Find values associated to this haystack
    pub fn find(&self, haystack: &str) -> Vec<&V> {
        let mut values = Vec::new();

        if self.regex.is_match(haystack) {
            for child in &self.children {
                values.extend(child.find(haystack));
            }
        }

        values
    }

    pub fn get(&self, regex: &str) -> Vec<&V> {
        let mut values = Vec::new();

        if regex.starts_with(self.regex.original.as_str()) {
            for child in &self.children {
                values.extend(child.get(regex));
            }
        }

        values
    }

    pub fn get_mut(&mut self, regex: &str) -> Vec<&mut V> {
        let mut values = Vec::new();

        if regex.starts_with(self.regex.original.as_str()) {
            for child in &mut self.children {
                values.extend(child.get_mut(regex));
            }
        }

        values
    }

    pub fn regex(&self) -> &str {
        self.regex.original.as_str()
    }

    /// Traces when finding a value
    // fn trace(&self, haystack: &str) -> Trace<V> {}

    /// Remove an item on this tree
    ///
    /// This method returns true if there is no more data so it can be cleaned up
    pub fn remove(mut self, id: &str) -> (Item<V>, Option<V>) {
        let mut removed = None;
        let mut children = Vec::new();

        for child in self.children {
            if removed.is_some() {
                children.push(child);
            } else {
                let (child, value) = child.remove(id);

                if value.is_some() {
                    removed = value;
                }

                if child.len() > 0 {
                    children.push(child);
                }
            }
        }

        if children.len() == 1 {
            return (children.pop().unwrap(), removed);
        }

        self.children = children;

        (Item::Node(self), removed)
    }

    /// Length of node
    pub fn len(&self) -> usize {
        let mut count = 0;

        for child in &self.children {
            count += child.len();
        }

        count
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
    pub fn cache(&mut self, mut left: u64, max_level: u64, current_level: u64) -> u64 {
        // Already cached
        if self.regex.compiled.is_none() {
            self.regex.compile();

            if self.regex.compiled.is_some() {
                left = left - 1;
            }
        }

        for child in &mut self.children {
            left = child.cache(left, max_level, current_level);
        }

        left
    }
}
