use crate::regex_radix_tree::{NodeItem, Node, Trace, Storage};
use std::marker::PhantomData;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct Leaf<T: NodeItem, S: Storage<T>> {
    storage: S,
    level: u64,
    prefix: String,
    prefix_compiled: Option<Regex>,
    phantom: PhantomData<T>,
}

impl<T: NodeItem, S: Storage<T>> Node<T, S> for Leaf<T, S> {
    fn insert(&mut self, item: T, _parent_prefix_size: u32) {
        self.storage.push(item)
    }

    fn find(&self, value: &str) -> Vec<&S> {
        if self.is_match(value) {
            return vec![&self.storage];
        }

        Vec::new()
    }

    fn trace(&self, value: &str) -> Trace<T, S> {
        let matched = self.is_match(value);
        let storage = if matched { Some(self.storage.clone()) } else { None };

        Trace::new(self.prefix.clone(), matched, self.storage.len() as u64, Vec::new(), storage)
    }

    fn remove(&mut self, id: &str) {
        self.storage.remove(id);
    }

    fn regex(&self) -> &str {
        self.prefix.as_str()
    }

    fn len(&self) -> usize {
        self.storage.len()
    }

    fn is_empty(&self) -> bool {
        self.storage.is_empty()
    }

    fn can_insert_item(&self, _prefix: &str, item: &T) -> bool {
        item.regex() == self.prefix
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

    fn box_clone(&self) -> Box<dyn Node<T, S>> {
        Box::new(self.clone())
    }
}

impl<T: NodeItem, S: Storage<T>> Leaf<T, S> {
    pub fn new(item: T, level: u64) -> Leaf<T, S> {
        let mut storage = S::new(item.regex());
        let prefix = item.regex().to_string();

        storage.push(item);

        Leaf {
            prefix,
            storage: storage,
            level,
            prefix_compiled: None,
            phantom: PhantomData,
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
