use crate::http::Request;
use crate::marker::StaticOrDynamic;
use crate::regex_radix_tree::{NodeItem, RegexRadixTree};
use crate::router::request_matcher::matcher_tree_storage::{ItemRoute, MatcherTreeStorage};
use crate::router::trace::TraceInfo;
use crate::router::{IpMatcher, RequestMatcher, Route, RouteData, RouterConfig, Trace};
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct HostRegexNodeItem<T: RouteData> {
    route: Route<T>,
    host_regex: String,
    ignore_case: bool,
}

impl<T: RouteData> NodeItem for HostRegexNodeItem<T> {
    fn regex(&self) -> &str {
        self.host_regex.as_str()
    }

    fn case_insensitive(&self) -> bool {
        self.ignore_case
    }
}

impl<T: RouteData> ItemRoute<T> for HostRegexNodeItem<T> {
    fn route(self) -> Route<T> {
        self.route
    }
}

type HostRegexTreeMatcher<T> = MatcherTreeStorage<T, HostRegexNodeItem<T>, IpMatcher<T>>;

#[derive(Debug, Clone)]
pub struct HostMatcher<T: RouteData> {
    static_hosts: HashMap<String, Box<dyn RequestMatcher<T>>>,
    regex_tree_rule: RegexRadixTree<HostRegexNodeItem<T>, HostRegexTreeMatcher<T>>,
    any_host: Box<dyn RequestMatcher<T>>,
    always_match_any_host: bool,
    count: usize,
}

impl<T: RouteData> RequestMatcher<T> for HostMatcher<T> {
    fn insert(&mut self, route: Route<T>) {
        self.count += 1;

        match route.host() {
            None => self.any_host.insert(route),
            Some(host) => match host {
                StaticOrDynamic::Static(static_host) => {
                    if static_host.is_empty() {
                        self.any_host.insert(route);

                        return;
                    }

                    if !self.static_hosts.contains_key(static_host) {
                        self.static_hosts.insert(static_host.clone(), HostMatcher::create_sub_matcher());
                    }

                    self.static_hosts.get_mut(static_host).unwrap().insert(route);
                }
                StaticOrDynamic::Dynamic(dynamic_host) => {
                    self.regex_tree_rule.insert(HostRegexNodeItem {
                        host_regex: dynamic_host.regex.clone(),
                        ignore_case: dynamic_host.ignore_case,
                        route,
                    });
                }
            },
        }
    }

    fn remove(&mut self, id: &str) -> bool {
        let mut removed = false;

        if self.any_host.remove(id) {
            self.count -= 1;

            return true;
        }

        if self.regex_tree_rule.remove(id) {
            self.count -= 1;

            return true;
        }

        self.static_hosts.retain(|_, matcher| {
            removed = removed || matcher.remove(id);

            matcher.len() > 0
        });

        if removed {
            self.count -= 1;
        }

        removed
    }

    fn match_request(&self, request: &Request) -> Vec<&Route<T>> {
        let mut routes = Vec::new();

        if let Some(host) = request.host() {
            let storages = self.regex_tree_rule.find(host);

            for storage in storages {
                routes.extend(storage.matcher.match_request(request));
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

    fn trace(&self, request: &Request) -> Vec<Trace<T>> {
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
            let node_trace = self.regex_tree_rule.trace(host);
            traces.push(HostRegexTreeMatcher::<T>::node_trace_to_router_trace(
                host,
                node_trace,
                request,
                Some(TraceInfo::HostRegex),
            ));

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

    fn cache(&mut self, limit: u64, level: u64) -> u64 {
        let mut new_limit = self.regex_tree_rule.cache(limit, level);

        for matcher in self.static_hosts.values_mut() {
            new_limit = matcher.cache(new_limit, level);
        }

        self.any_host.cache(new_limit, level)
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

impl<T: RouteData> Default for HostMatcher<T> {
    fn default() -> Self {
        HostMatcher {
            static_hosts: HashMap::new(),
            any_host: HostMatcher::create_sub_matcher(),
            count: 0,
            regex_tree_rule: RegexRadixTree::default(),
            always_match_any_host: false,
        }
    }
}

impl<T: RouteData> HostMatcher<T> {
    pub fn create_sub_matcher() -> Box<dyn RequestMatcher<T>> {
        Box::<IpMatcher<T>>::default()
    }

    pub fn new(config: RouterConfig) -> Self {
        HostMatcher {
            static_hosts: HashMap::new(),
            any_host: HostMatcher::create_sub_matcher(),
            count: 0,
            regex_tree_rule: RegexRadixTree::default(),
            always_match_any_host: config.always_match_any_host,
        }
    }
}
