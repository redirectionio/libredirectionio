use super::item::Item;
use super::leaf::Leaf;
use super::prefix::{common_prefix_char_size, get_prefix_with_char_size};
use super::regex::LazyRegex;
#[cfg(feature = "dot")]
use crate::dot::DotBuilder;
#[cfg(feature = "dot")]
use dot_graph::{Edge, Graph, Node as GraphNode};
use std::sync::Arc;

#[derive(Debug)]
pub struct Node<V> {
    pub(crate) regex: Arc<LazyRegex>,
    pub(crate) children: Vec<Item<V>>,
}

impl<V> Clone for Node<V>
where
    V: Clone,
{
    fn clone(&self) -> Self {
        Node {
            regex: self.regex.clone(),
            children: self.children.clone(),
        }
    }
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
                regex: Arc::new(LazyRegex::new_node(prefix, self.regex.ignore_case)),
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

                if !child.is_empty() {
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

    pub fn retain<F>(mut self, f: &F) -> Item<V>
    where
        F: Fn(&str, &mut V) -> bool,
    {
        let mut children = Vec::new();

        for child in self.children {
            let child = child.retain(f);

            if !child.is_empty() {
                children.push(child);
            }
        }

        if children.is_empty() {
            return Item::Empty(self.regex.ignore_case);
        }

        if children.len() == 1 {
            return children.pop().unwrap();
        }

        self.children = children;

        Item::Node(self)
    }

    /// Length of node
    pub fn len(&self) -> usize {
        let mut count = 0;

        for child in &self.children {
            count += child.len();
        }

        count
    }

    /// Length of node
    pub fn cached_len(&self) -> usize {
        let mut count = 0;

        if self.regex.compiled.is_some() {
            count += 1;
        }

        for child in &self.children {
            count += child.cached_len();
        }

        count
    }

    pub fn is_empty(&self) -> bool {
        for child in &self.children {
            if !child.is_empty() {
                return false;
            }
        }

        true
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
    pub fn cache(&mut self, mut left: u64, cache_level: u64, current_level: u64) -> u64 {
        // Already cached
        if cache_level == current_level && self.regex.compiled.is_none() {
            self.regex = Arc::new(self.regex.compile());

            if self.regex.compiled.is_some() {
                left -= 1;
            }
        }

        for child in &mut self.children {
            left = child.cache(left, cache_level, current_level + 1);
        }

        left
    }
}

#[cfg(feature = "dot")]
impl<V> DotBuilder for Node<V>
where
    V: DotBuilder,
{
    fn graph(&self, id: &mut u32, graph: &mut Graph) -> Option<String> {
        let node_name = format!("node_regex_{}", id);
        *id += 1;

        let mut node = GraphNode::new(&node_name).label(self.regex.original.as_str());

        if self.regex.compiled.is_some() {
            node = node.color(Some("green"));
        } else {
            node = node.color(Some("red"));
        }

        graph.add_node(node);

        for child in &self.children {
            if let Some(child) = child.graph(id, graph) {
                graph.add_edge(Edge::new(&node_name, &child, self.regex()));
            }
        }

        Some(node_name)
    }
}
