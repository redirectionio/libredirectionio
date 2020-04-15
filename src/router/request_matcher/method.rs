use crate::router::request_matcher::{RequestMatcher, HeaderMatcher};
use crate::router::{Route, RouteData};
use http::Request;
use std::collections::HashMap;

#[derive(Debug)]
pub struct MethodMatcher<T> {
    methods: HashMap<String, Box<dyn RequestMatcher<T>>>,
    any_method: Box<dyn RequestMatcher<T>>,
}

impl<T> RequestMatcher<T> for MethodMatcher<T> where T: RouteData {
    fn insert(&mut self, route: Route<T>) {
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

    fn remove(&mut self, id: &str) -> bool {
        let mut empty = self.any_method.remove(id);

        self.methods.retain(|_, matcher| {
            !matcher.remove(id)
        });

        empty && self.methods.is_empty()
    }

    fn match_request(&self, request: &Request<()>) -> Vec<&Route<T>> {
        let mut routes = self.any_method.match_request(request);

        if let Some(matcher) = self.methods.get(request.method().as_str()) {
            routes.extend(matcher.match_request(request));
        }

        routes
    }

    fn cache(&mut self, limit: u64, level: u64) -> u64 {
        let mut new_limit = self.any_method.cache(limit, level);

        for (_, matcher) in &mut self.methods {
            new_limit = matcher.cache(new_limit, level);
        }

        new_limit
    }
}

impl<T> MethodMatcher<T> where T: RouteData {
    pub fn new() -> MethodMatcher<T> {
        MethodMatcher {
            methods: HashMap::new(),
            any_method: MethodMatcher::create_sub_matcher(),
        }
    }

    pub fn create_sub_matcher() -> Box<dyn RequestMatcher<T>> {
        Box::new(HeaderMatcher::new())
    }
}
