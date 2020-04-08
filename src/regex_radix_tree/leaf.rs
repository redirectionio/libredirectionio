use crate::regex_radix_tree::{Node, Item};

pub struct Leaf<T> where T: Item {
    data: T,
    level: u64,
}

impl<T> Node<T> for Leaf<T> where T: Item {
    fn find(&self, value: &str) -> Vec<&T> {
        unimplemented!()
    }
}
