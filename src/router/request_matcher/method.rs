use super::super::request_matcher::HeaderMatcher;
use super::super::trace::TraceInfo;
use super::super::{Route, RouterConfig, Trace};
use crate::http::Request;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct MethodMatcher<T> {
    methods: HashMap<String, HeaderMatcher<T>>,
    exclude_methods: HashMap<Vec<String>, HeaderMatcher<T>>,
    any_method: HeaderMatcher<T>,
    count: usize,
    config: Arc<RouterConfig>,
}

impl<T> MethodMatcher<T> {
    pub fn new(config: Arc<RouterConfig>) -> Self {
        Self {
            methods: HashMap::new(),
            exclude_methods: HashMap::new(),
            any_method: HeaderMatcher::new(config.clone()),
            count: 0,
            config,
        }
    }

    pub fn insert(&mut self, route: Arc<Route<T>>) {
        self.count += 1;

        let config = self.config.clone();

        match route.methods() {
            None => self.any_method.insert(route),
            Some(methods) => {
                if methods.is_empty() {
                    self.any_method.insert(route);
                } else {
                    if route.exclude_methods().is_some() {
                        self.exclude_methods
                            .entry(methods.clone())
                            .or_insert_with(|| HeaderMatcher::new(config.clone()))
                            .insert(route.clone());

                        return;
                    }
                    for method in methods {
                        if !self.methods.contains_key(method) {
                            self.methods.insert(method.to_string(), HeaderMatcher::new(config.clone()));
                        }

                        self.methods.get_mut(method).unwrap().insert(route.clone());
                    }
                }
            }
        }
    }

    pub fn remove(&mut self, id: &str) -> Option<Arc<Route<T>>> {
        let mut removed = self.any_method.remove(id);

        if removed.is_some() {
            self.count -= 1;

            return removed;
        }

        self.methods.retain(|_, matcher| {
            if let Some(value) = matcher.remove(id) {
                removed = Some(value);
            }

            !matcher.is_empty()
        });

        self.exclude_methods.retain(|_, matcher| {
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

    pub fn batch_remove(&mut self, ids: &HashSet<String>) -> bool {
        self.any_method.batch_remove(ids);

        self.methods.retain(|_, matcher| {
            matcher.batch_remove(ids);

            !matcher.is_empty()
        });

        self.exclude_methods.retain(|_, matcher| {
            matcher.batch_remove(ids);

            !matcher.is_empty()
        });

        self.any_method.is_empty() && self.methods.is_empty() && self.exclude_methods.is_empty()
    }

    pub fn match_request(&self, request: &Request) -> Vec<Arc<Route<T>>> {
        let mut routes = self.any_method.match_request(request);

        if let Some(matcher) = self.methods.get(request.method()) {
            routes.extend(matcher.match_request(request));
        }

        for (methods, matcher) in &self.exclude_methods {
            if !methods.contains(&request.method().into()) {
                routes.extend(matcher.match_request(request));
            }
        }

        routes
    }

    pub fn trace(&self, request: &Request) -> Vec<Trace<T>> {
        let mut traces = self.any_method.trace(request);
        let request_method = request.method();
        let mut found = false;

        for (methods, matcher) in &self.exclude_methods {
            if !methods.contains(&request_method.into()) {
                found = true;
                let method_traces = matcher.trace(request);

                traces.push(Trace::new(
                    true,
                    true,
                    matcher.len() as u64,
                    method_traces,
                    TraceInfo::ExcludeMethods {
                        request: request_method.to_string(),
                        against: Some(methods.clone()),
                    },
                ));
            } else {
                traces.push(Trace::new(
                    false,
                    false,
                    matcher.len() as u64,
                    Vec::new(),
                    TraceInfo::ExcludeMethods {
                        request: request_method.to_string(),
                        against: Some(methods.clone()),
                    },
                ));
            }
        }

        for (method, matcher) in &self.methods {
            if method == request_method {
                found = true;
                let method_traces = matcher.trace(request);

                traces.push(Trace::new(
                    true,
                    true,
                    matcher.len() as u64,
                    method_traces,
                    TraceInfo::Method {
                        request: request_method.to_string(),
                        against: Some(method.clone()),
                    },
                ));
            } else {
                traces.push(Trace::new(
                    false,
                    false,
                    matcher.len() as u64,
                    Vec::new(),
                    TraceInfo::Method {
                        request: request_method.to_string(),
                        against: Some(method.clone()),
                    },
                ));
            }
        }

        if !found {
            traces.push(Trace::new(
                true,
                false,
                0,
                Vec::new(),
                TraceInfo::Method {
                    request: request_method.to_string(),
                    against: None,
                },
            ));
        }

        traces
    }

    pub fn cache(&mut self, limit: u64, level: u64) -> u64 {
        let mut new_limit = self.any_method.cache(limit, level);

        for matcher in self.methods.values_mut() {
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
