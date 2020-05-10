use crate::regex_radix_tree::Storage;
use std::fmt::Debug;

pub trait VecStorageItem: Debug + Clone + Send + Sync + 'static {
    fn id(&self) -> &str;
}

impl<T: VecStorageItem> Storage<T> for Vec<T> {
    fn push(&mut self, item: T) {
        self.push(item);
    }

    fn remove(&mut self, id: &str) -> bool {
        if let Some(index) = self.iter().position(|item| item.id() == id) {
            self.remove(index);

            return true;
        }

        false
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
