use crate::regex_radix_tree::item::Item;
use std::collections::hash_map::{Values, ValuesMut};

pub struct ItemIter<'a, V> {
    pub(crate) children: &'a [Item<V>],
    pub(crate) parent: Option<Box<ItemIter<'a, V>>>,
    pub(crate) values: Option<Values<'a, String, V>>,
}

pub struct ItemIterMut<'a, V> {
    pub(crate) children: &'a mut [Item<V>],
    pub(crate) parent: Option<Box<ItemIterMut<'a, V>>>,
    pub(crate) values: Option<ValuesMut<'a, String, V>>,
}

impl<It> Default for ItemIter<'_, It> {
    fn default() -> Self {
        ItemIter {
            children: &[],
            parent: None,
            values: None,
        }
    }
}

impl<It> Default for ItemIterMut<'_, It> {
    fn default() -> Self {
        ItemIterMut {
            children: &mut [],
            parent: None,
            values: None,
        }
    }
}

impl<'a, V> Iterator for ItemIter<'a, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.values {
            None => match self.children.first() {
                None => match self.parent.take() {
                    Some(parent) => {
                        // continue with the parent node
                        *self = *parent;
                        self.next()
                    }
                    None => None,
                },
                Some(Item::Empty(_)) => {
                    self.children = &self.children[1..];
                    self.next()
                }
                Some(Item::Leaf(item)) => {
                    self.children = &self.children[1..];
                    self.values = Some(item.values.values());

                    self.next()
                }
                Some(Item::Node(children)) => {
                    self.children = &self.children[1..];

                    // start iterating the child trees
                    *self = ItemIter {
                        children: children.children.as_slice(),
                        parent: Some(Box::new(std::mem::take(self))),
                        values: None,
                    };

                    self.next()
                }
            },
            Some(values) => match values.next() {
                None => {
                    self.values = None;
                    self.next()
                }
                Some(value) => Some(value),
            },
        }
    }
}

impl<'a, V> Iterator for ItemIterMut<'a, V> {
    type Item = &'a mut V;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.values {
            None => {
                let children = std::mem::take(&mut self.children);

                match children.split_first_mut() {
                    None => match self.parent.take() {
                        Some(parent) => {
                            // continue with the parent node
                            *self = *parent;
                            self.next()
                        }
                        None => None,
                    },
                    Some((Item::Empty(_), children)) => {
                        self.children = children;
                        self.next()
                    }
                    Some((Item::Leaf(item), children)) => {
                        self.children = children;
                        self.values = Some(item.values.values_mut());

                        self.next()
                    }
                    Some((Item::Node(item), children)) => {
                        self.children = children;

                        // start iterating the child trees
                        *self = ItemIterMut {
                            children: item.children.as_mut_slice(),
                            parent: Some(Box::new(std::mem::take(self))),
                            values: None,
                        };

                        self.next()
                    }
                }
            }
            Some(values) => match values.next() {
                None => {
                    self.values = None;
                    self.next()
                }
                Some(value) => Some(value),
            },
        }
    }
}
