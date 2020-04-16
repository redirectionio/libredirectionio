use crate::regex_radix_tree::Item;
use crate::router::request_matcher::RequestMatcher;
use crate::router::{Route, RouteData, Trace};
use http::Request;

#[derive(Debug)]
pub struct RegexItemMatcher<T> where T: RouteData {
    matcher: Box<dyn RequestMatcher<T>>,
    regex: String,
    id: String,
}

impl<T> Item for RegexItemMatcher<T> where T: RouteData {
    fn node_regex(&self) -> &str {
        self.regex.as_str()
    }

    fn id(&self) -> &str {
        self.id.as_str()
    }
}

impl<T> RequestMatcher<T> for RegexItemMatcher<T> where T: RouteData {
    fn insert(&mut self, route: Route<T>) {
        self.matcher.insert(route)
    }

    fn remove(&mut self, id: &str) -> Vec<Route<T>> {
        self.matcher.remove(id)
    }

    fn match_request(&self, request: &Request<()>) -> Vec<&Route<T>> {
        self.matcher.match_request(request)
    }

    fn trace(&self, request: &Request<()>) -> Vec<Trace> {
        self.matcher.trace(request)
    }

    fn cache(&mut self, limit: u64, level: u64) -> u64 {
        self.matcher.cache(limit, level)
    }

    fn len(&self) -> usize {
        self.matcher.len()
    }
}

impl<T> RegexItemMatcher<T> where T: RouteData {
    pub fn new(regex: String, id: String, matcher: Box<dyn RequestMatcher<T>>) -> RegexItemMatcher<T> {
        RegexItemMatcher {
            matcher,
            regex,
            id,
        }
    }
}
