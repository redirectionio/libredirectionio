use crate::regex_radix_tree::{RegexRadixTree, Trace as NodeTrace};
use crate::router::request_matcher::{RequestMatcher, RouteMatcher};
use crate::router::request_matcher::regex_item_matcher::RegexItemMatcher;
use crate::router::{Route, RouteData, Trace};
use crate::router::marker_string::StaticOrDynamic;
use http::Request;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct PathAndQueryMatcher<T: RouteData> {
    regex_tree_rule: RegexRadixTree<RegexItemMatcher<T>>,
    static_rules: HashMap<String, Box<dyn RequestMatcher<T>>>,
    count: usize,
}

impl<T: RouteData> RequestMatcher<T> for PathAndQueryMatcher<T> {
    fn insert(&mut self, route: Route<T>) {
        self.count += 1;

        match route.path_and_query() {
            StaticOrDynamic::Static(path) => {
                if self.static_rules.contains_key(path) {
                    self.static_rules.insert(path.clone(), PathAndQueryMatcher::create_sub_matcher());
                }

                self.static_rules.get_mut(path).unwrap().insert(route);
            },
            StaticOrDynamic::Dynamic(path) => {
                let mut item_matcher = PathAndQueryMatcher::create_item_matcher(path.regex.clone(), route.id().to_string());

                item_matcher.insert(route);
                self.regex_tree_rule.insert(item_matcher)
            }
        }
    }

    fn remove(&mut self, id: &str) -> Vec<Route<T>> {
        let mut removed_matchers = self.regex_tree_rule.remove(id);
        let mut routes = Vec::new();

        for matcher in &mut removed_matchers {
            routes.extend(matcher.remove(id));
        }

        self.static_rules.retain(|_, matcher| {
            routes.extend(matcher.remove(id));

            matcher.len() > 0
        });

        self.count -= routes.len();

        routes
    }

    fn match_request(&self, request: &Request<()>) -> Vec<&Route<T>> {
        let path = PathAndQueryMatcher::<T>::request_to_path(request);
        let matchers = self.regex_tree_rule.find(path.as_str());
        let mut routes = Vec::new();

        for matcher in matchers {
            routes.extend(matcher.match_request(request));
        }

        match self.static_rules.get(path.as_str()) {
            None => (),
            Some(static_matcher) => {
                routes.extend(static_matcher.match_request(request));
            }
        }

        routes
    }

    fn trace(&self, request: &Request<()>) -> Vec<Trace<T>> {
        let path = PathAndQueryMatcher::<T>::request_to_path(request);
        let node_trace = self.regex_tree_rule.trace(path.as_str());
        let mut traces = vec![PathAndQueryMatcher::<T>::node_trace_to_router_trace(node_trace, request)];

        let static_traces = match self.static_rules.get(path.as_str()) {
            None => Vec::new(),
            Some(static_matcher) => {
                static_matcher.trace(request)
            }
        };

        traces.push(Trace::new(
            "static_path".to_string(),
            !static_traces.is_empty(),
            true,
            static_traces.len() as u64,
            static_traces,
            Vec::new(),
        ));

        traces
    }

    fn cache(&mut self, limit: u64, level: u64) -> u64 {
        let mut new_limit = self.regex_tree_rule.cache(limit, level);

        for (_, matcher) in &mut self.static_rules {
            new_limit = matcher.cache(new_limit, level)
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

impl<T> PathAndQueryMatcher<T> where T: RouteData {
    pub fn new() -> PathAndQueryMatcher<T> {
        PathAndQueryMatcher {
            regex_tree_rule: RegexRadixTree::new(),
            static_rules: HashMap::new(),
            count: 0,
        }
    }

    pub fn request_to_path(request: &Request<()>) -> String {
        let mut path = request.uri().path().to_string();

        if request.uri().query().is_some() {
            path = [path, "?".to_string(), request.uri().query().unwrap().to_string()].join("");
        }

        path
    }

    fn node_trace_to_router_trace(trace: NodeTrace<RegexItemMatcher<T>>, request: &Request<()>) -> Trace<T> {
        let mut children = Vec::new();

        for child in trace.children {
            children.push(PathAndQueryMatcher::<T>::node_trace_to_router_trace(child, request));
        }

        for matcher in trace.items {
            children.extend(matcher.trace(request));
        }

        Trace::new(
            format!("Regex tree prefix {}", trace.regex),
            trace.matched,
            true,
            trace.count,
            Vec::new(),
            Vec::new(),
        )
    }

    fn create_item_matcher(path: String, id: String) -> RegexItemMatcher<T> {
        RegexItemMatcher::new(
            path,
            id,
            Box::new(RouteMatcher::new())
        )
    }

    fn create_sub_matcher() -> Box<dyn RequestMatcher<T>> {
        Box::new(RouteMatcher::new())
    }
}
