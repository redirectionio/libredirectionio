use crate::router::request_matcher::RequestMatcher;
use crate::router::{Route, RouteData};
use http::Request;

#[derive(Debug)]
pub struct RouteMatcher<T> where T: RouteData {
    routes: Vec<Route<T>>,
}

impl<T> RequestMatcher<T> for RouteMatcher<T> where T: RouteData {
    fn insert(&mut self, route: Route<T>) {
        self.routes.push(route)
    }

    fn remove(&mut self, id: &str) -> bool {
        self.routes.retain(|route| {
            route.id() != id
        });

        self.routes.is_empty()
    }

    fn match_request(&self, request: &Request<()>) -> Vec<&Route<T>> {
        self.routes.iter().collect::<Vec<_>>()
    }

    fn cache(&mut self, limit: u64, _level: u64) -> u64 {
        limit
    }
}

impl<T> RouteMatcher<T> where T: RouteData {
    pub fn new() -> RouteMatcher<T> {
        RouteMatcher {
            routes: Vec::new(),
        }
    }
}
