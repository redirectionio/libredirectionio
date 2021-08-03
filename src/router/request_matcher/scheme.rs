use crate::http::Request;
use crate::router::request_matcher::{HostMatcher, RequestMatcher};
use crate::router::trace::TraceInfo;
use crate::router::{Route, RouteData, RouterConfig, Trace};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SchemeMatcher<T: RouteData> {
    schemes: HashMap<String, Box<dyn RequestMatcher<T>>>,
    any_scheme: Box<dyn RequestMatcher<T>>,
    count: usize,
    config: RouterConfig,
}

impl<T: RouteData> RequestMatcher<T> for SchemeMatcher<T> {
    fn insert(&mut self, route: Route<T>) {
        self.count += 1;

        match route.scheme() {
            None => self.any_scheme.insert(route),
            Some(scheme) => {
                if scheme.is_empty() {
                    self.any_scheme.insert(route)
                } else {
                    if !self.schemes.contains_key(scheme) {
                        self.schemes
                            .insert(scheme.to_string(), SchemeMatcher::create_sub_matcher(self.config.clone()));
                    }

                    self.schemes.get_mut(scheme).unwrap().insert(route);
                }
            }
        }
    }

    fn remove(&mut self, id: &str) -> bool {
        let mut removed = false;

        if self.any_scheme.remove(id) {
            self.count -= 1;

            return true;
        }

        self.schemes.retain(|_, matcher| {
            removed = removed || matcher.remove(id);

            matcher.len() > 0
        });

        if removed {
            self.count -= 1;
        }

        removed
    }

    fn match_request(&self, request: &Request) -> Vec<&Route<T>> {
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

    fn trace(&self, request: &Request) -> Vec<Trace<T>> {
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

    fn cache(&mut self, limit: u64, level: u64) -> u64 {
        let mut new_limit = self.any_scheme.cache(limit, level);

        for matcher in self.schemes.values_mut() {
            new_limit = matcher.cache(new_limit, level);
        }

        new_limit
    }

    fn len(&self) -> usize {
        self.count
    }

    fn is_empty(&self) -> bool {
        self.count == 0
    }

    fn box_clone(&self) -> Box<dyn RequestMatcher<T>> {
        Box::new((*self).clone())
    }
}

impl<T: RouteData> SchemeMatcher<T> {
    pub fn create_sub_matcher(config: RouterConfig) -> Box<dyn RequestMatcher<T>> {
        Box::new(HostMatcher::new(config))
    }

    pub fn new(config: RouterConfig) -> Self {
        SchemeMatcher {
            schemes: HashMap::new(),
            any_scheme: SchemeMatcher::create_sub_matcher(config.clone()),
            config,
            count: 0,
        }
    }
}
