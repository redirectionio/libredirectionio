use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

#[cfg(feature = "dot")]
use dot_graph::{Edge, Graph, Node};

use super::super::{IpMatcher, Route, RouterConfig, Trace, trace::TraceInfo};
#[cfg(feature = "dot")]
use crate::dot::DotBuilder;
use crate::{
    http::Request,
    marker::StaticOrDynamic,
    regex_radix_tree::{Trace as TreeTrace, UniqueRegexTreeMap},
};

#[derive(Debug, Clone)]
pub struct HostMatcher<T> {
    static_hosts: HashMap<String, IpMatcher<T>>,
    regex_tree_rule: UniqueRegexTreeMap<IpMatcher<T>>,
    any_host: IpMatcher<T>,
    always_match_any_host: bool,
    count: usize,
    config: Arc<RouterConfig>,
}

impl<T> HostMatcher<T> {
    pub fn new(config: Arc<RouterConfig>) -> Self {
        HostMatcher {
            static_hosts: HashMap::new(),
            any_host: IpMatcher::new(config.clone()),
            count: 0,
            regex_tree_rule: UniqueRegexTreeMap::new(config.ignore_host_case),
            always_match_any_host: config.always_match_any_host,
            config,
        }
    }

    pub fn insert(&mut self, route: Arc<Route<T>>) {
        self.count += 1;

        match route.host() {
            None => self.any_host.insert(route.clone()),
            Some(host) => match host {
                StaticOrDynamic::Static(static_host) => {
                    if static_host.is_empty() {
                        self.any_host.insert(route.clone());

                        return;
                    }

                    if !self.static_hosts.contains_key(static_host) {
                        self.static_hosts.insert(static_host.clone(), IpMatcher::new(self.config.clone()));
                    }

                    self.static_hosts.get_mut(static_host).unwrap().insert(route.clone());
                }
                StaticOrDynamic::Dynamic(dynamic_host) => match self.regex_tree_rule.get_mut(dynamic_host.regex.as_str()) {
                    Some(matcher) => matcher.insert(route.clone()),
                    None => {
                        let mut matcher = IpMatcher::new(self.config.clone());
                        matcher.insert(route.clone());

                        self.regex_tree_rule.insert(dynamic_host.regex.as_str(), matcher);
                    }
                },
            },
        }
    }

    pub fn remove(&mut self, id: &str) -> Option<Arc<Route<T>>> {
        let mut removed = self.any_host.remove(id);

        if removed.is_some() {
            self.count -= 1;

            return removed;
        }

        self.static_hosts.retain(|_, matcher| {
            if let Some(value) = matcher.remove(id) {
                removed = Some(value);
            }

            !matcher.is_empty()
        });

        self.regex_tree_rule.retain(&|_, matcher| {
            matcher.remove(id);
            !matcher.is_empty()
        });

        if removed.is_some() {
            self.count -= 1;
        }

        removed
    }

    pub fn batch_remove(&mut self, ids: &HashSet<String>) -> bool {
        self.any_host.batch_remove(ids);

        self.static_hosts.retain(|_, matcher| {
            matcher.batch_remove(ids);

            !matcher.is_empty()
        });

        self.regex_tree_rule.retain(&|_, matcher| {
            matcher.batch_remove(ids);

            !matcher.is_empty()
        });

        self.any_host.is_empty() && self.static_hosts.is_empty() && self.regex_tree_rule.is_empty()
    }

    pub fn match_request(&self, request: &Request) -> Vec<Arc<Route<T>>> {
        let mut routes = Vec::new();

        if let Some(host) = request.host() {
            let matchers = self.regex_tree_rule.find(host);

            for matcher in matchers {
                routes.extend(matcher.match_request(request));
            }

            if let Some(matcher) = self.static_hosts.get(host) {
                routes.extend(matcher.match_request(request));
            }
        }

        if self.always_match_any_host || routes.is_empty() {
            routes.extend(self.any_host.match_request(request));
        }

        routes
    }

    pub fn trace(&self, request: &Request) -> Vec<Trace<T>> {
        let mut traces = Vec::new();
        let request_host = request.host().unwrap_or("");

        for (host, matcher) in &self.static_hosts {
            if host == request_host && request.host().is_some() {
                let host_traces = matcher.trace(request);

                traces.push(Trace::new(
                    true,
                    true,
                    matcher.len() as u64,
                    host_traces,
                    TraceInfo::HostStatic {
                        request: request_host.to_string(),
                        against: Some(host.clone()),
                    },
                ));
            } else {
                traces.push(Trace::new(
                    false,
                    false,
                    matcher.len() as u64,
                    Vec::new(),
                    TraceInfo::HostStatic {
                        request: request_host.to_string(),
                        against: Some(host.clone()),
                    },
                ));
            }
        }

        if let Some(host) = request.host() {
            let tree_trace = self.regex_tree_rule.trace(host);
            let trace = tree_trace_to_trace(host, tree_trace, request);
            traces.push(Trace::new(trace.matched, true, trace.count, vec![trace], TraceInfo::HostRegex));

            if !self.static_hosts.contains_key(host) {
                traces.push(Trace::new(
                    true,
                    false,
                    0,
                    Vec::new(),
                    TraceInfo::HostStatic {
                        request: request_host.to_string(),
                        against: None,
                    },
                ));
            }
        }

        if self.always_match_any_host || Trace::<T>::get_routes_from_traces(&traces).is_empty() {
            traces.extend(self.any_host.trace(request));
        }

        traces
    }

    pub fn cache(&mut self, limit: u64, level: u64) -> u64 {
        let mut new_limit = self.regex_tree_rule.cache(limit, Some(level));

        for matcher in self.static_hosts.values_mut() {
            new_limit = matcher.cache(new_limit, level);
        }

        for matcher in self.regex_tree_rule.iter_mut() {
            new_limit = matcher.cache(new_limit, level);
        }

        self.any_host.cache(new_limit, level)
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
}

fn tree_trace_to_trace<T>(haystack: &str, tree_trace: TreeTrace<IpMatcher<T>>, request: &Request) -> Trace<T> {
    let mut children = Vec::new();

    for child in tree_trace.children {
        children.push(tree_trace_to_trace(haystack, child, request));
    }

    for matcher in tree_trace.values {
        if tree_trace.matched {
            children.extend(matcher.trace(request));
        }
    }

    Trace::new(
        tree_trace.matched,
        true,
        tree_trace.count,
        children,
        TraceInfo::Regex {
            request: haystack.to_string(),
            against: tree_trace.regex,
        },
    )
}

#[cfg(feature = "dot")]
impl<V> DotBuilder for HostMatcher<V> {
    fn graph(&self, id: &mut u32, graph: &mut Graph) -> Option<String> {
        let node_name = format!("host_matcher_{}", id);
        *id += 1;
        graph.add_node(Node::new(&node_name));

        if let Some(key) = self.any_host.graph(id, graph) {
            graph.add_edge(Edge::new(&node_name, &key, "any host"));
        }

        for (host, matcher) in &self.static_hosts {
            if let Some(key) = matcher.graph(id, graph) {
                graph.add_edge(Edge::new(&node_name, &key, host));
            }
        }

        if let Some(key) = self.regex_tree_rule.graph(id, graph) {
            graph.add_edge(Edge::new(&node_name, &key, "regex host"));
        }

        Some(node_name)
    }
}
