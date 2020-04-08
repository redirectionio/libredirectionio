use std::hash::Hash;
use crate::regex_radix_tree::node::Node;

pub struct RegexRadixTree<T> where T: Hash {
    root: Box<dyn Node<T>>,
}

impl<T> RegexRadixTree<T> where T: Hash {
    pub fn insert(&mut self, key: &str, value: T) {

    }

    pub fn find(&self, value: &str) -> Vec<&T> {
        Vec::new()
    }

    pub fn remove(&mut self, value: &T) -> bool {
        false
    }
}
