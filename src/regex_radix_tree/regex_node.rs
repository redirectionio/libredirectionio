use regex::Regex;
use crate::regex_radix_tree::node::Node;

pub struct RegexNode<T> {
    empty: Option<RegexNodeBranch<T>>,
    children: Vec<RegexNodeBranch<T>>,
}

pub struct RegexNodeBranch<T> {
    regex: String,
    regex_compiled: Option<Regex>,
    node: Box<dyn Node<T>>,
    level: u64,
}
