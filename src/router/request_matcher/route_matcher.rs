use crate::router::request_matcher::RequestMatcher;
use crate::router::{Route, RouteData, Trace};
use http::Request;

#[derive(Debug)]
pub struct RouteMatcher<T> where T: RouteData {
    routes: Vec<Route<T>>,
}

impl<T> RequestMatcher<T> for RouteMatcher<T> where T: RouteData {
    fn insert(&mut self, route: Route<T>) {
        self.routes.push(route)
    }

    fn remove(&mut self, id: &str) -> Vec<Route<T>> {
        let mut i = 0;
        let mut removed = Vec::new();

        while i != self.routes.len() {
            let item = &mut self.routes[i];

            if item.id() == id {
                removed.push(self.routes.remove(i));
            } else {
                i += 1;
            }
        }

        removed
    }

    fn match_request(&self, request: &Request<()>) -> Vec<&Route<T>> {
        self.routes.iter().collect::<Vec<_>>()
    }

    fn trace(&self, request: &Request<()>) -> Vec<Trace> {
        let mut traces = Vec::new();

        for route in &self.routes {
            traces.push(Trace::new(
                route.id().to_string(),
                true,
                1,
                Vec::new(),
                Some(route.id().to_string()),
            ));
        }

        traces
    }

    fn cache(&mut self, limit: u64, _level: u64) -> u64 {
        limit
    }

    fn len(&self) -> usize {
        self.routes.len()
    }
}

impl<T> RouteMatcher<T> where T: RouteData {
    pub fn new() -> RouteMatcher<T> {
        RouteMatcher {
            routes: Vec::new(),
        }
    }
}
