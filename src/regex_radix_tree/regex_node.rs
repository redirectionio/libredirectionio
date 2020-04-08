use regex::Regex;
use crate::regex_radix_tree::{Node, Item};
use crate::regex_radix_tree::leaf::Leaf;
use crate::regex_radix_tree::prefix::{common_prefix_char_size, get_prefix_with_char_size};

pub struct RegexNode<T> where T: Item {
    prefix: String,
    prefix_compiled: Option<Regex>,
    level: u64,
    children: Vec<Box<dyn Node<T>>>,
}

impl<T> Node<T> for RegexNode<T> where T: Item {
    fn insert(&mut self, item: T) {
        let item_regex = item.node_regex();

        // for each children node
        for i in 0 .. self.children.len() {
            let regex_child = self.children[i].regex();
            let prefix_size = common_prefix_char_size(item_regex.as_str(), regex_child);

            if prefix_size == 0 {
                continue;
            }

            let prefix = get_prefix_with_char_size(item_regex.as_str(), prefix_size);

            if prefix == regex_child {
                self.children[i].insert(item);

                return;
            }

            let current_item = self.children.remove(i);

            self.children.push(Box::new(RegexNode::new(
                Box::new(Leaf::new(item, self.level + 1)),
                current_item,
                prefix.to_string(),
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
                            items.extend(child.find(value).unwrap_or(Vec::new()));
                        }
                    }
                }

                values
            },
            false => None,
        }
    }

    fn is_match(&self, value: &str) -> bool {
        match self.prefix_compiled.as_ref() {
            Some(regex) => regex.is_match(value),
            None => self.create_regex().is_match(value),
        }
    }

    fn regex(&self) -> &str {
        self.prefix.as_str()
    }

    fn cache(&mut self, limit: u64, level: u64) -> u64 {
        if self.level == level {
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
            prefix,
            prefix_compiled: None,
            children: vec![first, second]
        }
    }

    fn create_regex(&self) -> Regex {
        // @TODO Change this to error handler
        let regex = ["^", self.prefix.as_str()].join("");
        Regex::new(regex.as_str()).expect("Cannot create regex")
    }
}
