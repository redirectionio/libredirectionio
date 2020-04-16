use crate::router::request_matcher::{RequestMatcher, HeaderMatcher};
use crate::router::{Route, RouteData, Trace};
use http::Request;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct MethodMatcher<T: RouteData> {
    methods: HashMap<String, Box<dyn RequestMatcher<T>>>,
    any_method: Box<dyn RequestMatcher<T>>,
    count: usize,
}

impl<T: RouteData> RequestMatcher<T> for MethodMatcher<T> {
    fn insert(&mut self, route: Route<T>) {
        self.count += 1;

        match route.methods() {
            None => self.any_method.insert(route),
            Some(methods) => {
                for method in methods {
                    if !self.methods.contains_key(method) {
                        self.methods.insert(method.to_string(), MethodMatcher::create_sub_matcher());
                    }

                    self.methods.get_mut(method).unwrap().insert(route.clone());
                }
            }
        }
    }

    fn remove(&mut self, id: &str) -> Vec<Route<T>> {
        let mut removed = Vec::new();

        removed.extend(self.any_method.remove(id));

        self.methods.retain(|_, matcher| {
            removed.extend(matcher.remove(id));

            matcher.len() > 0
        });

        self.count -= removed.len();

        removed
    }

    fn match_request(&self, request: &Request<()>) -> Vec<&Route<T>> {
        let mut routes = self.any_method.match_request(request);

        if let Some(matcher) = self.methods.get(request.method().as_str()) {
            routes.extend(matcher.match_request(request));
        }

        routes
    }

    fn trace(&self, request: &Request<()>) -> Vec<Trace<T>> {
        let any_traces= self.any_method.trace(request);
        let mut traces = Vec::new();
        let request_method = request.method().as_str();

        traces.push(Trace::new(
            "Any method".to_string(),
            true,
            true,
            self.any_method.len() as u64,
            any_traces,
            Vec::new(),
        ));

        for (method, matcher) in &self.methods {
            if method == request_method {
                let method_traces = matcher.trace(request);

                traces.push(Trace::new(
                    format!("Method {}", method),
                    true,
                    true,
                    matcher.len() as u64,
                    method_traces,
                    Vec::new(),
                ));
            } else {
                traces.push(Trace::new(
                    format!("Method {}", method),
                    false,
                    false,
                    matcher.len() as u64,
                    Vec::new(),
                    Vec::new(),
                ));
            }
        }

        if !self.methods.contains_key(request_method) {
            traces.push(Trace::new(
                format!("Method {}", request_method),
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
        let mut new_limit = self.any_method.cache(limit, level);

        for (_, matcher) in &mut self.methods {
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

impl<T> MethodMatcher<T> where T: RouteData {
    pub fn new() -> MethodMatcher<T> {
        MethodMatcher {
            methods: HashMap::new(),
            any_method: MethodMatcher::create_sub_matcher(),
            count: 0,
        }
    }

    pub fn create_sub_matcher() -> Box<dyn RequestMatcher<T>> {
        Box::new(HeaderMatcher::new())
    }
}
