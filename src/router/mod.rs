pub mod request_matcher;
mod route;
mod marker_string;
mod trace;
mod transformer;

pub use route::{RouteData, Route};
pub use marker_string::{StaticOrDynamic, Marker};
pub use trace::{RouteTrace, Trace};
pub use request_matcher::{PathAndQueryMatcher, SchemeMatcher, RequestMatcher};
pub use transformer::{Transform, Transformer, Camelize, Uppercase, Underscorize, Slice, Replace, Dasherize, Lowercase};

use http::Request;

#[derive(Debug)]
pub struct Router<T: RouteData> {
    matcher: SchemeMatcher<T>,
}

impl<T: RouteData> Default for Router<T> {
    fn default() -> Self {
        Router {
            matcher: SchemeMatcher::default()
        }
    }
}

impl<T: RouteData> Router<T> {
    pub fn insert(&mut self, route: Route<T>) {
        self.matcher.insert(route);
    }

    pub fn remove(&mut self, id: &str) -> Vec<Route<T>> {
        self.matcher.remove(id)
    }

    pub fn match_request(&self, request: &Request<()>) -> Vec<&Route<T>> {
        self.matcher.match_request(request)
    }

    pub fn len(&self) -> usize {
        self.matcher.len()
    }

    pub fn is_empty(&self) -> bool {
        self.matcher.is_empty()
    }

    pub fn trace_request(&self, request: &Request<()>) -> Vec<Trace<T>> {
        self.matcher.trace(request)
    }

    pub fn get_route(&self, request: &Request<()>) -> Option<&Route<T>> {
        let mut routes = self.match_request(request);

        if routes.is_empty() {
            return None;
        }

        routes.sort_by(|a, b| a.priority().cmp(&b.priority()));

        match routes.get(0) {
            None => None,
            Some(&route) => {
                Some(route)
            }
        }
    }

    pub fn get_trace(&self, request: &Request<()>) -> RouteTrace<T> {
        let traces = self.trace_request(request);
        let mut routes_traces = Trace::get_routes_from_traces(&traces);
        let mut routes = Vec::new();

        for &route in &routes_traces {
            routes.push(route.clone());
        }

        routes_traces.sort_by(|&a, &b| a.priority().cmp(&b.priority()));



        let final_route = match routes_traces.first() {
            None => None,
            Some(&route) => Some(route.clone())
        };

        RouteTrace::new(traces, routes, final_route)
    }

    pub fn cache(&mut self, limit: u64) {
        let mut prev_cache_limit = limit;
        let mut level = 0;

        while prev_cache_limit > 0 {
            let next_cache_limit = self.matcher.cache(prev_cache_limit, level);

            if next_cache_limit == prev_cache_limit && level > 5 {
                break;
            }

            level += 1;
            prev_cache_limit = next_cache_limit;
        }
    }
}
