use crate::router::request_matcher::{RequestMatcher, MethodMatcher};
use crate::router::{Route, RouteData};
use http::Request;
use std::collections::HashMap;

#[derive(Debug)]
pub struct HostMatcher<T> {
    hosts: HashMap<String, Box<dyn RequestMatcher<T>>>,
    any_host: Box<dyn RequestMatcher<T>>,
}

impl<T> RequestMatcher<T> for HostMatcher<T> where T: RouteData {
    fn insert(&mut self, route: Route<T>) {
        match route.host() {
            None => self.any_host.insert(route),
            Some(host) => {
                if !self.hosts.contains_key(host) {
                    self.hosts.insert(host.to_string(), HostMatcher::create_sub_matcher());
                }

                self.hosts.get_mut(host).unwrap().insert(route);
            }
        }
    }

    fn remove(&mut self, id: &str) -> bool {
        let mut empty = self.any_host.remove(id);

        self.hosts.retain(|_, matcher| {
            !matcher.remove(id)
        });

        empty && self.hosts.is_empty()
    }

    fn match_request(&self, request: &Request<()>) -> Vec<&Route<T>> {
        if let Some(host) = request.uri().host() {
            if let Some(matcher) = self.hosts.get(host) {
                let rules = matcher.match_request(request);

                if !rules.is_empty() {
                    return rules;
                }
            }
        }

        self.any_host.match_request(request)
    }

    fn cache(&mut self, limit: u64, level: u64) -> u64 {
        let mut new_limit = limit;

        for (_, matcher) in &mut self.hosts {
            new_limit = matcher.cache(new_limit, level);
        }

        self.any_host.cache(new_limit, level)
    }
}

impl<T> HostMatcher<T> where T: RouteData {
    pub fn new() -> HostMatcher<T> {
        HostMatcher {
            hosts: HashMap::new(),
            any_host: HostMatcher::create_sub_matcher(),
        }
    }

    pub fn create_sub_matcher() -> Box<dyn RequestMatcher<T>> {
        Box::new(MethodMatcher::new())
    }
}
