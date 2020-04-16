use crate::router::{Route, RouteData};

pub struct Trace {
    name: String,
    matched: bool,
    count: u64,
    children: Vec<Trace>,
    route_id: Option<String>,
}

impl Trace {
    pub fn new(name: String, matched: bool, count: u64, children: Vec<Trace>, route_id: Option<String>) -> Trace {
        Trace {
            name,
            matched,
            route_id,
            children,
            count,
        }
    }
}
