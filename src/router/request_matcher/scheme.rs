use super::super::trace::TraceInfo;
use super::super::{HostMatcher, Route, RouterConfig, Trace};
use crate::http::Request;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct SchemeMatcher<T> {
    schemes: HashMap<String, HostMatcher<T>>,
    any_scheme: HostMatcher<T>,
    count: usize,
    config: Arc<RouterConfig>,
}

impl<T> SchemeMatcher<T> {
    pub fn new(config: Arc<RouterConfig>) -> Self {
        SchemeMatcher {
            schemes: HashMap::new(),
            any_scheme: HostMatcher::new(config.clone()),
            config,
            count: 0,
        }
    }

    pub fn insert(&mut self, route: Arc<Route<T>>) {
        self.count += 1;

        match route.scheme() {
            None => self.any_scheme.insert(route),
            Some(scheme) => {
                if scheme.is_empty() {
                    self.any_scheme.insert(route)
                } else {
                    if !self.schemes.contains_key(scheme) {
                        self.schemes.insert(scheme.to_string(), HostMatcher::new(self.config.clone()));
                    }

                    self.schemes.get_mut(scheme).unwrap().insert(route);
                }
            }
        }
    }

    pub fn remove(&mut self, id: &str) -> Option<Arc<Route<T>>> {
        let mut removed = self.any_scheme.remove(id);

        if removed.is_some() {
            self.count -= 1;

            return removed;
        }

        self.schemes.retain(|_, matcher| {
            if let Some(value) = matcher.remove(id) {
                removed = Some(value);
            }

            !matcher.is_empty()
        });

        if removed.is_some() {
            self.count -= 1;
        }

        removed
    }

    pub fn match_request(&self, request: &Request) -> Vec<Arc<Route<T>>> {
        let mut routes = self.any_scheme.match_request(request);

        match request.scheme() {
            None => (),
            Some(scheme) => {
                if let Some(matcher) = self.schemes.get(scheme) {
                    routes.extend(matcher.match_request(request));
                }
            }
        }

        routes
    }

    pub fn trace(&self, request: &Request) -> Vec<Trace<T>> {
        let mut traces = self.any_scheme.trace(request);
        let request_scheme = request.scheme().unwrap_or("");

        for (scheme, matcher) in &self.schemes {
            if scheme == request_scheme && !request_scheme.is_empty() {
                let scheme_traces = matcher.trace(request);

                traces.push(Trace::new(
                    true,
                    true,
                    matcher.len() as u64,
                    scheme_traces,
                    TraceInfo::Scheme {
                        request: request_scheme.to_string(),
                        against: Some(scheme.clone()),
                    },
                ));
            } else {
                traces.push(Trace::new(
                    false,
                    false,
                    matcher.len() as u64,
                    Vec::new(),
                    TraceInfo::Scheme {
                        request: request_scheme.to_string(),
                        against: Some(scheme.clone()),
                    },
                ));
            }
        }

        if !request_scheme.is_empty() && !self.schemes.contains_key(request_scheme) {
            traces.push(Trace::new(
                true,
                false,
                0,
                Vec::new(),
                TraceInfo::Scheme {
                    request: request_scheme.to_string(),
                    against: Some(request_scheme.to_string()),
                },
            ));
        }

        traces
    }

    pub fn cache(&mut self, limit: u64, level: u64) -> u64 {
        let mut new_limit = self.any_scheme.cache(limit, level);

        for matcher in self.schemes.values_mut() {
            new_limit = matcher.cache(new_limit, level);
        }

        new_limit
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
}
