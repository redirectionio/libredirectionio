pub struct Trace {
    regex: String,
    count: u64,
    matched: bool,
    children: Vec<Trace>,
}

impl Trace {
    pub fn new(regex: String, matched: bool, count: u64, children: Vec<Trace>) -> Trace {
        Trace {
            regex,
            matched,
            children,
            count,
        }
    }
}
