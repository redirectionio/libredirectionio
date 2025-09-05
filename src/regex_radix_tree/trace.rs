use super::{item::Item, leaf::Leaf, node::Node};

#[derive(Debug, Clone)]
pub struct Trace<'a, V> {
    pub(crate) regex: String,
    pub(crate) count: u64,
    pub(crate) matched: bool,
    pub(crate) children: Vec<Trace<'a, V>>,
    pub(crate) values: Vec<&'a V>,
}

impl<V> Leaf<V> {
    pub fn trace(&self, haystack: &str) -> Trace<'_, V> {
        let matched = self.regex.is_match(haystack);

        Trace {
            regex: self.regex.original.clone(),
            matched,
            count: self.values.len() as u64,
            children: Vec::new(),
            values: self.values.values().collect(),
        }
    }
}
impl<V> Node<V> {
    pub fn trace(&self, haystack: &str) -> Trace<'_, V> {
        let mut children = Vec::new();
        let matched = self.regex.is_match(haystack);

        if matched {
            for child in &self.children {
                children.push(child.trace(haystack));
            }
        }

        Trace {
            regex: self.regex.original.clone(),
            matched,
            count: self.len() as u64,
            children,
            values: Vec::new(),
        }
    }
}

impl<V> Item<V> {
    pub fn trace(&self, haystack: &str) -> Trace<'_, V> {
        match self {
            Item::Empty(_) => Trace {
                regex: "".to_string(),
                matched: true,
                count: 0,
                children: Vec::new(),
                values: Vec::new(),
            },
            Item::Node(node) => node.trace(haystack),
            Item::Leaf(leaf) => leaf.trace(haystack),
        }
    }
}
