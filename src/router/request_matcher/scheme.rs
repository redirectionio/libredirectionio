use crate::router::request_matcher::{RequestMatcher, HostMatcher};
use crate::router::{Route, RouteData, Trace};
use http::Request;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SchemeMatcher<T: RouteData> {
    schemes: HashMap<String, Box<dyn RequestMatcher<T>>>,
    any_scheme: Box<dyn RequestMatcher<T>>,
    count: usize,
}

impl<T> RequestMatcher<T> for SchemeMatcher<T> where T: RouteData {
    fn insert(&mut self, route: Route<T>) {
        self.count += 1;

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

    fn remove(&mut self, id: &str) -> Vec<Route<T>> {
        let mut removed = Vec::new();

        removed.extend(self.any_scheme.remove(id));

        self.schemes.retain(|_, matcher| {
            removed.extend(matcher.remove(id));

            matcher.len() > 0
        });

        self.count -= removed.len();

        removed
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

    fn trace(&self, request: &Request<()>) -> Vec<Trace<T>> {
        let any_traces = self.any_scheme.trace(request);
        let mut traces = Vec::new();
        let request_scheme = match request.uri().scheme() {
            None => "",
            Some(scheme) => scheme.as_str(),
        };

        traces.push(Trace::new(
            "Any scheme".to_string(),
            true,
            true,
            self.any_scheme.len() as u64,
            any_traces,
            Vec::new(),
        ));

        for (scheme, matcher) in &self.schemes {
            if scheme == request_scheme && request_scheme != "" {
                let scheme_traces = matcher.trace(request);

                traces.push(Trace::new(
                    format!("Scheme {}", scheme),
                    true,
                    true,
                    matcher.len() as u64,
                    scheme_traces,
                    Vec::new(),
                ));
            } else {
                traces.push(Trace::new(
                    format!("Scheme {}", scheme),
                    false,
                    false,
                    matcher.len() as u64,
                    Vec::new(),
                    Vec::new(),
                ));
            }
        }

        if request_scheme != "" && !self.schemes.contains_key(request_scheme) {
            traces.push(Trace::new(
                format!("Scheme {}", request_scheme),
                true,
                false,
                0,
                Vec::new(),
                Vec::new(),
            ));
        }

        traces
    }

    fn cache(&mut self, limit: u64, level: u64) -> u64 {
        let mut new_limit = self.any_scheme.cache(limit, level);

        for (_, matcher) in &mut self.schemes {
            new_limit = matcher.cache(new_limit, level);
        }

        new_limit
    }

    fn len(&self) -> usize {
        self.count
    }

    fn box_clone(&self) -> Box<dyn RequestMatcher<T>> {
        Box::new((*self).clone())
    }
}

impl<T> SchemeMatcher<T> where T: RouteData {
    pub fn new() -> SchemeMatcher<T> {
        SchemeMatcher {
            schemes: HashMap::new(),
            any_scheme: SchemeMatcher::create_sub_matcher(),
            count: 0,
        }
    }

    pub fn create_sub_matcher() -> Box<dyn RequestMatcher<T>> {
        Box::new(HostMatcher::new())
    }
}
