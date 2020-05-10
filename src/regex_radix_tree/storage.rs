use std::fmt::Debug;
use crate::regex_radix_tree::Item;

pub trait Storage<T: Item>: Debug + Clone + Send + Sync + 'static  {
    fn push(&mut self, item: T);

    fn remove(&mut self, id: &str);

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool;

    fn new(regex: &str) -> Self;
}

impl<T: Item> Storage<T> for Vec<T> {
    fn push(&mut self, item: T) {
        self.push(item);
    }

    fn remove(&mut self, id: &str) {
        if let Some(index) = self.iter().position(|item| item.id() == id ) {
            self.remove(index);
        }
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn new(_regex: &str) -> Self {
        Vec::new()
    }
}
