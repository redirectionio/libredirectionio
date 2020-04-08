use crate::regex_radix_tree::Item;

pub trait Node<T> where T: Item {
    fn find(&self, value: &str) -> Vec<&T>;
}
