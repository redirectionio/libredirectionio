use super::super::trace::TraceInfo;
use super::super::{Route, RouterConfig, Trace};
use crate::http::Request;
use crate::marker::StaticOrDynamic;
use crate::regex_radix_tree_new::RegexTreeMap;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug)]
pub struct PathAndQueryMatcher<T> {
    regex_tree_rule: RegexTreeMap<Route<T>>,
    static_rules: HashMap<String, HashMap<String, Route<T>>>,
    count: usize,
}

impl<T> PathAndQueryMatcher<T> {
    pub fn new(config: Arc<RouterConfig>) -> Self {
        PathAndQueryMatcher {
            regex_tree_rule: RegexTreeMap::new(config.ignore_path_and_query_case),
            static_rules: HashMap::new(),
            count: 0,
        }
    }

    pub fn insert(&mut self, route: Route<T>) {
        self.count += 1;

        match route.path_and_query() {
            StaticOrDynamic::Static(path) => {
                if !self.static_rules.contains_key(path) {
                    self.static_rules.insert(path.clone(), HashMap::new());
                }

                self.static_rules.get_mut(path).unwrap().insert(route.id().to_string(), route);
            }
            StaticOrDynamic::Dynamic(path) => {
                self.regex_tree_rule.insert(path.regex.as_str(), route.id(), route);
            }
        }
    }

    pub fn remove(&mut self, id: &str) -> Option<Route<T>> {
        match self.regex_tree_rule.remove(id) {
            None => (),
            Some(route) => {
                self.count -= 1;

                return Some(route);
            }
        }

        let mut removed = None;

        self.static_rules.retain(|_, matcher| {
            if removed.is_some() {
                return true;
            }

            removed = matcher.remove(id);

            matcher.len() > 0
        });

        if removed.is_some() {
            self.count -= 1;
        }

        removed
    }

    pub fn match_request(&self, request: &Request) -> Vec<&Route<T>> {
        let path = request.path_and_query();
        let mut routes = self.regex_tree_rule.find(path.as_str());

        match self.static_rules.get(path.as_str()) {
            None => (),
            Some(static_storage) => {
                routes.extend(static_storage.values().collect());
            }
        }

        routes
    }

    pub fn trace(&self, request: &Request) -> Vec<Trace<T>> {
        let path = request.path_and_query();
        let mut traces = Vec::new();

        // @TODO Convert tree trace to router trace
        // let node_trace = self.regex_tree_rule.trace(path.as_str());
        //     vec![PathAndQueryRegexTreeMatcher::<T>::node_trace_to_router_trace(
        //     path.as_str(),
        //     node_trace,
        //     request,
        //     Some(TraceInfo::PathAndQueryRegex),
        // )];

        let static_traces = match self.static_rules.get(path.as_str()) {
            None => Vec::new(),
            Some(static_matcher) => static_matcher.trace(request),
        };

        traces.push(Trace::new(
            !static_traces.is_empty(),
            true,
            self.static_rules.len() as u64,
            static_traces,
            TraceInfo::PathAndQueryStatic { request: path },
        ));

        traces
    }

    pub fn cache(&mut self, limit: u64, level: u64) -> u64 {
        let mut new_limit = self.regex_tree_rule.cache(limit);

        for matcher in self.static_rules.values_mut() {
            new_limit = matcher.cache(new_limit, level)
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
