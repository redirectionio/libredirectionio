use crate::regex_radix_tree::{Node, Item};
use regex::Regex;

#[derive(Debug)]
pub struct Leaf<T> where T: Item {
    data: Vec<T>,
    level: u64,
    prefix: String,
    prefix_compiled: Option<Regex>,
}

impl<T> Node<T> for Leaf<T> where T: Item {
    fn insert(&mut self, item: T, _parent_prefix_size: u32) {
        self.data.push(item)
    }

    fn find(&self, value: &str) -> Option<Vec<&T>> {
        match self.is_match(value) {
            true => Some(self.data.iter().collect::<Vec<_>>()),
            false => None,
        }
    }

    fn remove(&mut self, id: &str) -> bool {
        self.data.retain(|item| {
            item.id() != id
        });

        self.data.is_empty()
    }

    fn regex(&self) -> &str {
        self.prefix.as_str()
    }

    fn can_insert_item(&self, _prefix: &str, item: &T) -> bool {
        item.node_regex() == self.prefix
    }

    fn incr_level(&mut self) {
        self.level += 1
    }

    fn cache(&mut self, limit: u64, level: u64) -> u64 {
        if self.level != level {
            return limit;
        }

        if limit == 0 {
            return limit;
        }

        // @TODO handle error
        self.prefix_compiled = Some(self.create_regex());

        limit - 1
    }
}

impl<T> Leaf<T> where T: Item {
    pub fn new(item: T, level: u64) -> Leaf<T> {
        Leaf {
            prefix: item.node_regex().to_string(),
            data: vec![item],
            level,
            prefix_compiled: None,
        }
    }

    fn is_match(&self, value: &str) -> bool {
        match self.prefix_compiled.as_ref() {
            Some(regex) => regex.is_match(value),
            None => self.create_regex().is_match(value),
        }
    }

    fn create_regex(&self) -> Regex {
        // @TODO Change this to error handler
        let regex = ["^", self.prefix.as_str(), "$"].join("");
        Regex::new(regex.as_str()).expect("Cannot create regex")
    }
}
