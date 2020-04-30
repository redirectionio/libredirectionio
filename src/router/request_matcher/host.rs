use crate::router::request_matcher::{RequestMatcher, MethodMatcher};
use crate::router::{Route, RouteData, Trace};
use http::Request;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct HostMatcher<T: RouteData> {
    hosts: HashMap<String, Box<dyn RequestMatcher<T>>>,
    any_host: Box<dyn RequestMatcher<T>>,
    count: usize,
}

impl<T: RouteData> RequestMatcher<T> for HostMatcher<T> {
    fn insert(&mut self, route: Route<T>) {
        self.count += 1;

        match route.host() {
            None => self.any_host.insert(route),
            Some(host) => {
                if host.is_empty() {
                    self.any_host.insert(route);

                    return;
                }

                if !self.hosts.contains_key(host) {
                    self.hosts.insert(host.to_string(), HostMatcher::create_sub_matcher());
                }

                self.hosts.get_mut(host).unwrap().insert(route);
            }
        }
    }

    fn remove(&mut self, id: &str) -> Vec<Route<T>> {
        let mut removed = Vec::new();

        removed.extend(self.any_host.remove(id));

        self.hosts.retain(|_, matcher| {
            removed.extend(matcher.remove(id));

            matcher.len() > 0
        });

        self.count -= removed.len();

        removed
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

    fn trace(&self, request: &Request<()>) -> Vec<Trace<T>> {
        let mut traces = Vec::new();
        let request_host = request.uri().host().unwrap_or("");

        for (host, matcher) in &self.hosts {
            if host == request_host && request_host != "" {
                let host_traces = matcher.trace(request);

                traces.push(Trace::new(
                    format!("Host {}", host),
                    true,
                    true,
                    matcher.len() as u64,
                    host_traces,
                    Vec::new(),
                ));
            } else {
                traces.push(Trace::new(
                    format!("Host {}", host),
                    false,
                    false,
                    matcher.len() as u64,
                    Vec::new(),
                    Vec::new(),
                ));
            }
        }

        if request_host != "" && !self.hosts.contains_key(request_host) {
            traces.push(Trace::new(
                format!("Host {}", request_host),
                true,
                false,
                0,
                Vec::new(),
                Vec::new(),
            ));
        }

        if Trace::<T>::get_routes_from_traces(&traces).is_empty() {
            return traces;
        }

        let any_traces = self.any_host.trace(request);

        traces.push(Trace::new(
            "Any host".to_string(),
            true,
            true,
            self.any_host.len() as u64,
            any_traces,
            Vec::new(),
        ));

        traces
    }

    fn cache(&mut self, limit: u64, level: u64) -> u64 {
        let mut new_limit = limit;

        for (_, matcher) in &mut self.hosts {
            new_limit = matcher.cache(new_limit, level);
        }

        self.any_host.cache(new_limit, level)
    }

    fn len(&self) -> usize {
        self.count
    }

    fn box_clone(&self) -> Box<dyn RequestMatcher<T>> {
        Box::new((*self).clone())
    }
}

impl<T> HostMatcher<T> where T: RouteData {
    pub fn new() -> HostMatcher<T> {
        HostMatcher {
            hosts: HashMap::new(),
            any_host: HostMatcher::create_sub_matcher(),
            count: 0,
        }
    }

    pub fn create_sub_matcher() -> Box<dyn RequestMatcher<T>> {
        Box::new(MethodMatcher::new())
    }
}