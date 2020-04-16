use crate::regex_radix_tree::RegexRadixTree;
use crate::router::request_matcher::{RequestMatcher, PathAndQueryMatcher};
use crate::router::{Route, RouteData, Trace};
use http::Request;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct HeaderMatcher<T> where T: RouteData {
    any_header: Box<dyn RequestMatcher<T>>,
}

impl<T> RequestMatcher<T> for HeaderMatcher<T> where T: RouteData {
    fn insert(&mut self, route: Route<T>) {
        self.any_header.insert(route);
    }

    fn remove(&mut self, id: &str) -> Vec<Route<T>> {
        self.any_header.remove(id)
    }

    fn match_request(&self, request: &Request<()>) -> Vec<&Route<T>> {
        self.any_header.match_request(request)
    }

    fn trace(&self, request: &Request<()>) -> Vec<Trace<T>> {
        self.any_header.trace(request)
    }

    fn cache(&mut self, limit: u64, level: u64) -> u64 {
        self.any_header.cache(limit, level)
    }

    fn len(&self) -> usize {
        self.any_header.len()
    }

    fn box_clone(&self) -> Box<dyn RequestMatcher<T>> {
        Box::new((*self).clone())
    }
}


impl<T> HeaderMatcher<T> where T: RouteData {
    pub fn new() -> HeaderMatcher<T> {
        HeaderMatcher {
            any_header: HeaderMatcher::create_sub_matcher(),
        }
    }

    pub fn create_sub_matcher() -> Box<dyn RequestMatcher<T>> {
        Box::new(PathAndQueryMatcher::new())
    }
}
