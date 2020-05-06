use crate::regex_radix_tree::{Item, Node, Trace};
use regex::Regex;

#[derive(Debug, Clone)]
pub struct Leaf<T: Item>
where
    T: Item,
{
    data: Vec<T>,
    level: u64,
    prefix: String,
    prefix_compiled: Option<Regex>,
}

impl<T: Item> Node<T> for Leaf<T> {
    fn insert(&mut self, item: T, _parent_prefix_size: u32) {
        self.data.push(item)
    }

    fn find(&self, value: &str) -> Vec<&T> {
        if self.is_match(value) {
            return self.data.iter().collect::<Vec<_>>();
        }

        Vec::new()
    }

    fn trace(&self, value: &str) -> Trace<T> {
        let matched = self.is_match(value);
        let items = if matched { self.data.clone() } else { Vec::new() };

        Trace::new(self.prefix.clone(), matched, self.data.len() as u64, Vec::new(), items)
    }

    fn remove(&mut self, id: &str) -> Vec<T> {
        let mut i = 0;
        let mut removed = Vec::new();

        while i != self.data.len() {
            let item = &mut self.data[i];

            if item.id() == id {
                removed.push(self.data.remove(i));
            } else {
                i += 1;
            }
        }

        removed
    }

    fn regex(&self) -> &str {
        self.prefix.as_str()
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
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

    fn box_clone(&self) -> Box<dyn Node<T>> {
        Box::new(self.clone())
    }
}

impl<T: Item> Leaf<T> {
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
