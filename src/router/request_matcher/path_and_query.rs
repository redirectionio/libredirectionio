use super::super::trace::TraceInfo;
use super::super::{Route, RouterConfig, Trace};
#[cfg(feature = "dot")]
use crate::dot::DotBuilder;
use crate::http::Request;
use crate::marker::StaticOrDynamic;
use crate::regex_radix_tree::{RegexTreeMap, Trace as TreeTrace};
#[cfg(feature = "dot")]
use dot_graph::{Edge, Graph, Node};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct PathAndQueryMatcher<T> {
    regex_tree_rule: RegexTreeMap<Arc<Route<T>>>,
    static_rules: HashMap<String, HashMap<String, Arc<Route<T>>>>,
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

    pub fn insert(&mut self, route: Arc<Route<T>>) {
        self.count += 1;

        match route.path_and_query() {
            StaticOrDynamic::Static(path) => {
                if !self.static_rules.contains_key(path) {
                    self.static_rules.insert(path.clone(), HashMap::new());
                }

                self.static_rules
                    .get_mut(path)
                    .unwrap()
                    .insert(route.id().to_string(), route.clone());
            }
            StaticOrDynamic::Dynamic(path) => {
                self.regex_tree_rule.insert(path.regex.as_str(), route.id(), route.clone());
            }
        }
    }

    pub fn batch_remove(&mut self, ids: &HashSet<String>) -> bool {
        self.static_rules.retain(|_, matcher| {
            matcher.retain(|id, _| !ids.contains(id));

            !matcher.is_empty()
        });

        self.regex_tree_rule.retain(&|id, _| !ids.contains(id));

        self.static_rules.is_empty() && self.regex_tree_rule.is_empty()
    }

    pub fn remove(&mut self, id: &str) -> Option<Arc<Route<T>>> {
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

            !matcher.is_empty()
        });

        if removed.is_some() {
            self.count -= 1;
        }

        removed
    }

    pub fn match_request(&self, request: &Request) -> Vec<Arc<Route<T>>> {
        let path = request.path_and_query();
        let mut routes: Vec<Arc<Route<T>>> = self
            .regex_tree_rule
            .find(path.as_str())
            .iter()
            .map(|route| (*route).clone())
            .collect();

        match self.static_rules.get(path.as_str()) {
            None => (),
            Some(static_storage) => {
                routes.extend(static_storage.values().cloned().collect::<Vec<Arc<Route<T>>>>());
            }
        }

        routes
    }

    pub fn trace(&self, request: &Request) -> Vec<Trace<T>> {
        let path = request.path_and_query();
        let trace = tree_trace_to_trace(path.as_str(), self.regex_tree_rule.trace(path.as_str()));

        let mut traces = vec![Trace::new(
            trace.matched,
            true,
            trace.count,
            vec![trace],
            TraceInfo::PathAndQueryRegex,
        )];

        let static_traces = match self.static_rules.get(path.as_str()) {
            None => Vec::new(),
            Some(routes) => {
                vec![Trace::new(
                    true,
                    true,
                    routes.len() as u64,
                    Vec::new(),
                    TraceInfo::Storage {
                        routes: routes.values().cloned().collect::<Vec<Arc<Route<T>>>>(),
                    },
                )]
            }
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
        self.regex_tree_rule.cache(limit, Some(level))
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
}

fn tree_trace_to_trace<T>(haystack: &str, tree_trace: TreeTrace<Arc<Route<T>>>) -> Trace<T> {
    let mut children = Vec::new();

    for child in tree_trace.children {
        children.push(tree_trace_to_trace(haystack, child));
    }

    if !tree_trace.values.is_empty() {
        children.push(Trace::new(
            tree_trace.matched,
            true,
            tree_trace.values.len() as u64,
            Vec::new(),
            TraceInfo::Storage {
                routes: if tree_trace.matched {
                    tree_trace.values.iter().map(|r| (*r).clone()).collect::<Vec<Arc<Route<T>>>>()
                } else {
                    Vec::new()
                },
            },
        ))
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
impl<V> DotBuilder for PathAndQueryMatcher<V> {
    fn graph(&self, id: &mut u32, graph: &mut Graph) -> Option<String> {
        let node_name = format!("path_matcher_{}", id);
        *id += 1;
        graph.add_node(Node::new(&node_name).label("path matcher"));

        if let Some(key) = self.regex_tree_rule.graph(id, graph) {
            graph.add_edge(Edge::new(&node_name, &key, "regex tree"));
        }

        let static_node_name = format!("static_matcher_{}", id);
        graph.add_node(Node::new(static_node_name.as_str()));
        graph.add_edge(Edge::new(&node_name, &static_node_name, "static rules"));

        Some(node_name)
    }
}
