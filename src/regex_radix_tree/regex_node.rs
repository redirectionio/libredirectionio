use crate::regex_radix_tree::leaf::Leaf;
use crate::regex_radix_tree::prefix::{common_prefix_char_size, get_prefix_with_char_size};
use crate::regex_radix_tree::{Node, NodeItem, Storage, Trace};
use regex::Regex;

#[derive(Debug, Clone)]
pub struct RegexNode<T: NodeItem, S: Storage<T>> {
    prefix: Option<String>,
    prefix_compiled: Option<Regex>,
    level: u64,
    children: Vec<Box<dyn Node<T, S>>>,
    count: usize,
}

impl<T: NodeItem, S: Storage<T>> Node<T, S> for RegexNode<T, S> {
    fn insert(&mut self, item: T, parent_prefix_size: u32) {
        self.count += 1;

        let item_regex = item.regex();

        // for each children node
        for i in 0..self.children.len() {
            let regex_child = self.children[i].regex();
            let prefix_size = common_prefix_char_size(item_regex, regex_child);

            if prefix_size == parent_prefix_size {
                continue;
            }

            let prefix = get_prefix_with_char_size(item_regex, prefix_size);

            if self.children[i].can_insert_item(prefix.as_str(), &item) {
                self.children[i].insert(item, prefix_size);

                return;
            }

            let mut current_item = self.children.remove(i);
            current_item.incr_level();

            self.children.push(Box::new(RegexNode::new(
                Box::new(Leaf::new(item, self.level + 2)),
                current_item,
                prefix,
                self.level + 1,
            )));

            return;
        }

        self.children.push(Box::new(Leaf::new(item, self.level + 1)));
    }

    fn find(&self, value: &str) -> Vec<&S> {
        let mut values = Vec::new();

        if self.is_match(value) {
            for child in &self.children {
                values.extend(child.find(value));
            }
        }

        values
    }

    fn trace(&self, value: &str) -> Trace<T, S> {
        let mut children = Vec::new();
        let matched = self.is_match(value);

        if matched {
            for child in &self.children {
                children.push(child.trace(value));
            }
        }

        Trace::new(
            self.prefix.clone().unwrap_or_else(|| "".to_string()),
            matched,
            self.count as u64,
            children,
            None,
        )
    }

    fn remove(&mut self, id: &str) -> bool {
        let mut i = 0;
        let mut removed = false;

        while i != self.children.len() {
            let child = &mut self.children[i];
            let prev_len = child.len();

            removed = child.remove(id);

            let new_len = child.len();

            self.count += new_len;
            self.count -= prev_len;

            if new_len == 0 {
                self.children.remove(i);
            } else {
                i += 1;
            }
        }

        removed
    }

    fn regex(&self) -> &str {
        match self.prefix.as_ref() {
            None => "",
            Some(prefix) => prefix,
        }
    }

    fn len(&self) -> usize {
        self.count
    }

    fn is_empty(&self) -> bool {
        self.count == 0
    }

    fn can_insert_item(&self, prefix: &str, _item: &T) -> bool {
        match self.prefix.as_ref() {
            None => true,
            Some(prefix_regex) => prefix_regex == prefix,
        }
    }

    fn incr_level(&mut self) {
        self.level += 1;

        for child in &mut self.children {
            child.incr_level();
        }
    }

    fn cache(&mut self, limit: u64, level: u64) -> u64 {
        if self.level == level && self.prefix.is_some() {
            self.prefix_compiled = Some(self.create_regex());

            return limit - 1;
        }

        let mut new_limit = limit;

        for child in &mut self.children {
            new_limit = child.cache(new_limit, level);
        }

        new_limit
    }

    fn box_clone(&self) -> Box<dyn Node<T, S>> {
        Box::new(self.clone())
    }
}

impl<T: NodeItem, S: Storage<T>> Default for RegexNode<T, S> {
    fn default() -> Self {
        RegexNode {
            level: 0,
            prefix: None,
            prefix_compiled: None,
            children: Vec::new(),
            count: 0,
        }
    }
}

impl<T: NodeItem, S: Storage<T>> RegexNode<T, S> {
    pub fn new(first: Box<dyn Node<T, S>>, second: Box<dyn Node<T, S>>, prefix: String, level: u64) -> RegexNode<T, S> {
        RegexNode {
            level,
            prefix: Some(prefix),
            prefix_compiled: None,
            count: first.len() + second.len(),
            children: vec![first, second],
        }
    }

    fn is_match(&self, value: &str) -> bool {
        if self.prefix.is_none() {
            return true;
        }

        match self.prefix_compiled.as_ref() {
            Some(regex) => regex.is_match(value),
            None => self.create_regex().is_match(value),
        }
    }

    #[allow(clippy::trivial_regex)]
    fn create_regex(&self) -> Regex {
        match self.prefix.as_ref() {
            None => Regex::new(""),
            Some(prefix) => {
                let regex = ["^", prefix.as_str()].join("");

                Regex::new(regex.as_str())
            }
        }
        .expect("Cannot create regex")
    }
}
