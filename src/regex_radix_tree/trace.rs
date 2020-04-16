use crate::regex_radix_tree::Item;

pub struct Trace<T: Item> {
    pub regex: String,
    pub count: u64,
    pub matched: bool,
    pub children: Vec<Trace<T>>,
    pub items: Vec<T>,
}

impl<T: Item> Trace<T> {
    pub fn new(regex: String, matched: bool, count: u64, children: Vec<Trace<T>>, items: Vec<T>) -> Trace<T> {
        Trace {
            regex,
            matched,
            children,
            count,
            items,
        }
    }
}
