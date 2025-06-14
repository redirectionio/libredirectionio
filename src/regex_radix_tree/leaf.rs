use std::{collections::HashMap, sync::Arc};

#[cfg(feature = "dot")]
use dot_graph::{Edge, Graph, Node as GraphNode};

use super::{item::Item, node::Node, prefix::common_prefix};
#[cfg(feature = "dot")]
use crate::dot::DotBuilder;
use crate::regex::LazyRegex;

#[derive(Debug)]
pub struct Leaf<V> {
    pub(crate) values: HashMap<String, V>,
    pub(crate) regex: Arc<LazyRegex>,
}

impl<V> Clone for Leaf<V>
where
    V: Clone,
{
    fn clone(&self) -> Self {
        Leaf {
            values: self.values.clone(),
            regex: self.regex.clone(),
        }
    }
}

impl<V> Leaf<V> {
    pub fn new(regex: &str, id: String, item: V, ignore_case: bool) -> Self {
        let mut values = HashMap::new();
        values.insert(id, item);

        Leaf {
            values,
            regex: Arc::new(LazyRegex::new_leaf(regex, ignore_case)),
        }
    }

    /// Insert a new item into this node
    pub fn insert(mut self, regex: &str, id: String, item: V) -> Item<V> {
        if regex == self.regex.original.as_str() {
            self.values.insert(id, item);

            return Item::Leaf(self);
        }

        let prefix = common_prefix(self.regex.original.as_str(), regex);
        let mut leaf_values = HashMap::new();
        leaf_values.insert(id, item);

        let leaf = Item::Leaf(Leaf {
            values: leaf_values,
            regex: Arc::new(LazyRegex::new_leaf(regex, self.regex.ignore_case)),
        });

        Item::Node(Node {
            regex: Arc::new(LazyRegex::new_node(prefix, self.regex.ignore_case)),
            children: vec![Item::Leaf(self), leaf],
        })
    }

    /// Find values associated to this haystack
    pub fn find(&self, haystack: &str) -> Vec<&V> {
        if self.regex.is_match(haystack) {
            return self.values.values().collect();
        }

        Vec::new()
    }

    pub fn get(&self, regex: &str) -> Vec<&V> {
        if self.regex.original.as_str() == regex {
            return self.values.values().collect();
        }

        Vec::new()
    }

    pub fn get_mut(&mut self, regex: &str) -> Vec<&mut V> {
        if self.regex.original.as_str() == regex {
            return self.values.values_mut().collect();
        }

        Vec::new()
    }

    /// Remove an item on this tree
    ///
    /// This method returns true if there is no more data so it can be cleaned up
    pub fn remove(mut self, id: &str) -> (Item<V>, Option<V>) {
        let removed = self.values.remove(id);

        match removed {
            None => (Item::Leaf(self), None),
            Some(value) => {
                if self.values.is_empty() {
                    (Item::Empty(self.regex.ignore_case), Some(value))
                } else {
                    (Item::Leaf(self), Some(value))
                }
            }
        }
    }
    pub fn retain<F>(mut self, f: &F) -> Item<V>
    where
        F: Fn(&str, &mut V) -> bool,
    {
        self.values.retain(|k, v| f(k, v));

        if self.values.is_empty() {
            Item::Empty(self.regex.ignore_case)
        } else {
            Item::Leaf(self)
        }
    }

    /// Length of node
    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn cached_len(&self) -> usize {
        if self.regex.compiled.is_some() {
            return 1;
        }

        0
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn regex(&self) -> &str {
        self.regex.original.as_str()
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
    pub fn cache(&mut self, left: u64) -> u64 {
        // Already cached
        if self.regex.compiled.is_some() {
            return left;
        }

        self.regex = Arc::new(self.regex.compile());

        if self.regex.compiled.is_some() {
            return left - 1;
        }

        left
    }
}

#[cfg(feature = "dot")]
impl<V> DotBuilder for Leaf<V>
where
    V: DotBuilder,
{
    fn graph(&self, id: &mut u32, graph: &mut Graph) -> Option<String> {
        let node_name = format!("leaf_regex_{}", id);
        *id += 1;

        let mut node = GraphNode::new(&node_name).label(self.regex.original.as_str());

        if self.regex.compiled.is_some() {
            node = node.color(Some("green"));
        } else {
            node = node.color(Some("red"));
        }

        graph.add_node(node);

        for (key, value) in &self.values {
            if let Some(value_key) = value.graph(id, graph) {
                graph.add_edge(Edge::new(&node_name, &value_key, key));
            }
        }

        Some(node_name)
    }
}
