use crate::router::request_matcher::{RequestMatcher, HostMatcher};
use crate::router::{Route, RouteData};
use http::Request;
use std::collections::HashMap;

#[derive(Debug)]
pub struct SchemeMatcher<T> {
    schemes: HashMap<String, Box<dyn RequestMatcher<T>>>,
    any_scheme: Box<dyn RequestMatcher<T>>,
}

impl<T> RequestMatcher<T> for SchemeMatcher<T> where T: RouteData {
    fn insert(&mut self, route: Route<T>) {
        match route.scheme() {
            None => self.any_scheme.insert(route),
            Some(scheme) => {
                if !self.schemes.contains_key(scheme) {
                    self.schemes.insert(scheme.to_string(), SchemeMatcher::create_sub_matcher());
                }

                self.schemes.get_mut(scheme).unwrap().insert(route);
            }
        }
    }

    fn remove(&mut self, id: &str) -> bool {
        let mut empty = self.any_scheme.remove(id);

        self.schemes.retain(|_, matcher| {
            !matcher.remove(id)
        });

        empty && self.schemes.is_empty()
    }

    fn match_request(&self, request: &Request<()>) -> Vec<&Route<T>> {
        let mut routes = self.any_scheme.match_request(request);

        match request.uri().scheme() {
            None => (),
            Some(scheme) => {
                if let Some(matcher) = self.schemes.get(scheme.as_str()) {
                    routes.extend(matcher.match_request(request));
                }
            }
        }

        routes
    }

    fn cache(&mut self, limit: u64, level: u64) -> u64 {
        let mut new_limit = self.any_scheme.cache(limit, level);

        for (_, matcher) in &mut self.schemes {
            new_limit = matcher.cache(new_limit, level);
        }

        new_limit
    }
}

impl<T> SchemeMatcher<T> where T: RouteData {
    pub fn new() -> SchemeMatcher<T> {
        SchemeMatcher {
            schemes: HashMap::new(),
            any_scheme: SchemeMatcher::create_sub_matcher(),
        }
    }

    pub fn create_sub_matcher() -> Box<dyn RequestMatcher<T>> {
        Box::new(HostMatcher::new())
    }
}
