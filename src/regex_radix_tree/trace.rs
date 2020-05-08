use crate::regex_radix_tree::{Item, Storage};
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct Trace<T: Item, S: Storage<T>> {
    pub regex: String,
    pub count: u64,
    pub matched: bool,
    pub children: Vec<Trace<T, S>>,
    pub storage: S,
    phantom: PhantomData<T>,
}

impl<T: Item, S: Storage<T>> Trace<T, S> {
    pub fn new(regex: String, matched: bool, count: u64, children: Vec<Trace<T, S>>, storage: S) -> Trace<T, S> {
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
