use std::fmt::Debug;

pub trait Storage<T: Debug + Clone + Send + Sync + 'static>: Debug + Clone + Send + Sync + 'static {
    fn push(&mut self, item: T);

    fn remove(&mut self, id: &str) -> bool;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool;

    fn new(regex: &str) -> Self;
}
