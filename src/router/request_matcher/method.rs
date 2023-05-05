use crate::http::Request;
use crate::router::request_matcher::{HeaderMatcher, RequestMatcher};
use crate::router::trace::TraceInfo;
use crate::router::{Route, RouteData, Trace};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct MethodMatcher<T: RouteData> {
    methods: HashMap<String, Box<dyn RequestMatcher<T>>>,
    exclude_methods: HashMap<Vec<String>, Box<dyn RequestMatcher<T>>>,
    any_method: Box<dyn RequestMatcher<T>>,
    count: usize,
}

impl<T: RouteData> RequestMatcher<T> for MethodMatcher<T> {
    fn insert(&mut self, route: Route<T>) {
        self.count += 1;

        match route.methods() {
            None => self.any_method.insert(route),
            Some(methods) => {
                if methods.is_empty() {
                    self.any_method.insert(route);
                } else {
                    if route.exclude_methods().is_some() {
                        self.exclude_methods
                            .entry(methods.clone())
                            .or_insert_with(|| MethodMatcher::create_sub_matcher())
                            .insert(route.clone());

                        return;
                    }
                    for method in methods {
                        if !self.methods.contains_key(method) {
                            self.methods.insert(method.to_string(), MethodMatcher::create_sub_matcher());
                        }

                        self.methods.get_mut(method).unwrap().insert(route.clone());
                    }
                }
            }
        }
    }

    fn remove(&mut self, id: &str) -> bool {
        let mut removed = false;

        if self.any_method.remove(id) {
            self.count -= 1;

            return true;
        }

        self.methods.retain(|_, matcher| {
            removed = removed || matcher.remove(id);

            matcher.len() > 0
        });

        self.exclude_methods.retain(|_, matcher| {
            removed = removed || matcher.remove(id);

            matcher.len() > 0
        });

        if removed {
            self.count -= 1;
        }

        removed
    }

    fn match_request(&self, request: &Request) -> Vec<&Route<T>> {
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

    fn trace(&self, request: &Request) -> Vec<Trace<T>> {
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

    fn cache(&mut self, limit: u64, level: u64) -> u64 {
        let mut new_limit = self.any_method.cache(limit, level);

        for matcher in self.methods.values_mut() {
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

impl<T: RouteData> Default for MethodMatcher<T> {
    fn default() -> Self {
        MethodMatcher {
            methods: HashMap::new(),
            exclude_methods: HashMap::new(),
            any_method: MethodMatcher::create_sub_matcher(),
            count: 0,
        }
    }
}

impl<T: RouteData> MethodMatcher<T> {
    pub fn create_sub_matcher() -> Box<dyn RequestMatcher<T>> {
        Box::<HeaderMatcher<T>>::default()
    }
}
