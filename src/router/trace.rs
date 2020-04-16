use crate::router::{Route, RouteData};

pub struct Trace<T: RouteData> {
    name: String,
    matched: bool,
    executed: bool,
    count: u64,
    children: Vec<Trace<T>>,
    routes: Vec<Route<T>>,
}

impl<T: RouteData> Trace<T> {
    pub fn new(name: String, matched: bool, executed: bool, count: u64, children: Vec<Trace<T>>, routes: Vec<Route<T>>) -> Trace<T> {
        Trace {
            name,
            matched,
            executed,
            children,
            count,
            routes,
        }
    }

    pub fn get_routes_from_traces(traces: &Vec<Trace<T>>) -> Vec<&Route<T>> {
        let mut routes = Vec::new();

        for trace in traces {
            routes.extend(trace.routes.iter().collect::<Vec<_>>());

            if !trace.children.is_empty() {
                routes.extend(Trace::get_routes_from_traces(&trace.children));
            }
        }

        routes
    }
}
