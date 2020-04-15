use regex::Regex;
use crate::regex_radix_tree::{Node, Item};
use crate::regex_radix_tree::leaf::Leaf;
use crate::regex_radix_tree::prefix::{common_prefix_char_size, get_prefix_with_char_size};

#[derive(Debug)]
pub struct RegexNode<T> where T: Item {
    prefix: Option<String>,
    prefix_compiled: Option<Regex>,
    level: u64,
    children: Vec<Box<dyn Node<T>>>,
}

impl<T> Node<T> for RegexNode<T> where T: Item {
    fn insert(&mut self, item: T, parent_prefix_size: u32) {
        let item_regex = item.node_regex();

        // for each children node
        for i in 0 .. self.children.len() {
            let regex_child= self.children[i].regex();
            let prefix_size= common_prefix_char_size(item_regex, regex_child);

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

    fn find(&self, value: &str) -> Option<Vec<&T>> {
        match self.is_match(value) {
            true => {
                let mut values = None;

                for child in &self.children {
                    match &mut values {
                        None => {
                            values = child.find(value);
                        },
                        Some(items) => {
                            match child.find(value) {
                                None => (),
                                Some(new_values) => items.extend(new_values),
                            }
                        }
                    }
                }

                values
            },
            false => None,
        }
    }

    fn remove(&mut self, id: &str) -> bool {
        let mut i = 0;

        while i != self.children.len() {
            let child = &mut self.children[i];

            if child.remove(id) {
                self.children.remove(i);
            } else {
                i += 1;
            }
        }

        self.children.is_empty()
    }

    fn regex(&self) -> &str {
        match self.prefix.as_ref() {
            None => "",
            Some(prefix) => prefix
        }
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
        if self.level == level && !self.prefix.is_none() {
            self.prefix_compiled = Some(self.create_regex());

            return limit - 1;
        }

        let mut new_limit = limit;

        for child in &mut self.children {
            new_limit = child.cache(new_limit, level);
        }

        new_limit
    }
}

impl<T> RegexNode<T> where T: Item {
    pub fn new(first: Box<dyn Node<T>>, second: Box<dyn Node<T>>, prefix: String, level: u64) -> RegexNode<T> {
        RegexNode {
            level,
            prefix: Some(prefix),
            prefix_compiled: None,
            children: vec![first, second]
        }
    }

    pub fn new_empty() -> RegexNode<T> {
        RegexNode {
            level: 0,
            prefix: None,
            prefix_compiled: None,
            children: Vec::new(),
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

    fn create_regex(&self) -> Regex {
        match self.prefix.as_ref() {
            None => Regex::new(""),
            Some(prefix) => {
                let regex = ["^", prefix.as_str()].join("");

                Regex::new(regex.as_str())
            },
        }.expect("Cannot create regex")
    }
}
