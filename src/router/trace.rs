use serde::{Deserialize, Serialize};
use crate::router::{Route, RouteData};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RouteTrace<T: RouteData> {
    traces: Vec<Trace<T>>,
    routes: Vec<Route<T>>,
    final_route: Option<Route<T>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Trace<T: RouteData> {
    name: String,
    matched: bool,
    executed: bool,
    count: u64,
    children: Vec<Trace<T>>,
    routes: Vec<Route<T>>,
}

impl<T: RouteData> RouteTrace<T> {
    pub fn new(traces: Vec<Trace<T>>, routes: Vec<Route<T>>, final_route: Option<Route<T>>) -> RouteTrace<T> {
        RouteTrace {
            traces,
            routes,
            final_route,
        }
    }
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

    pub fn get_routes_from_traces(traces: &[Trace<T>]) -> Vec<&Route<T>> {
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
