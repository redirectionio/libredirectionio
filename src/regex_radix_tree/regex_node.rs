use regex::Regex;
use crate::regex_radix_tree::{Node, Item};

pub struct RegexNode<T> where T: Item {
    empty: Option<RegexNodeBranch<T>>,
    children: Vec<RegexNodeBranch<T>>,
}

struct RegexNodeBranch<T> where T: Item {
    regex: String,
    regex_compiled: Option<Regex>,
    node: Box<dyn Node<T>>,
    level: u64,
}

impl<T> Node<T> for RegexNode<T> where T: Item {
    fn find(&self, value: &str) -> Vec<&T> {
        unimplemented!()
    }
}
