use crate::regex_radix_tree::{Node, Item};

pub struct RegexRadixTree<T> where T: Item {
    root: Option<Box<dyn Node<T>>>,
}

impl<T> RegexRadixTree<T> where T: Item {
    pub fn new() -> RegexRadixTree<T> {
        RegexRadixTree {
            root: None,
        }
    }

    pub fn insert(&mut self, value: T) {
        let regex = value.node_regex();
    }

    pub fn find(&self, value: &str) -> Option<Vec<&T>> {
        self.root.as_ref()?.find(value)
    }

    pub fn remove(&mut self, id: String) -> bool {
        false
    }
}
