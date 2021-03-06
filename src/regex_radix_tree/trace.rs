use crate::regex_radix_tree::{NodeItem, Storage};
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct Trace<T: NodeItem, S: Storage<T>> {
    pub regex: String,
    pub count: u64,
    pub matched: bool,
    pub children: Vec<Trace<T, S>>,
    pub storage: Option<S>,
    phantom: PhantomData<T>,
}

impl<T: NodeItem, S: Storage<T>> Trace<T, S> {
    pub fn new(regex: String, matched: bool, count: u64, children: Vec<Trace<T, S>>, storage: Option<S>) -> Trace<T, S> {
        Trace {
            regex,
            matched,
            children,
            count,
            storage,
            phantom: PhantomData,
        }
    }
}
