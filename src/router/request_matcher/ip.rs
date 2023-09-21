use super::super::route_ip::RouteIp;
use super::super::trace::TraceInfo;
use super::super::{MethodMatcher, Route, RouterConfig, Trace};
use crate::http::Request;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct IpMatcher<T> {
    matchers: HashMap<RouteIp, MethodMatcher<T>>,
    no_matcher: MethodMatcher<T>,
    count: usize,
    config: Arc<RouterConfig>,
}

impl<T> IpMatcher<T> {
    pub fn new(config: Arc<RouterConfig>) -> Self {
        IpMatcher {
            matchers: HashMap::new(),
            no_matcher: MethodMatcher::new(config.clone()),
            count: 0,
            config,
        }
    }

    pub fn insert(&mut self, route: Arc<Route<T>>) {
        self.count += 1;

        let config = self.config.clone();

        match route.ips() {
            Some(ips) => {
                for ip in ips {
                    self.matchers
                        .entry(ip.clone())
                        .or_insert_with(|| MethodMatcher::new(config.clone()))
                        .insert(route.clone());
                }
            }
            None => {
                self.no_matcher.insert(route);
            }
        }
    }

    pub fn remove(&mut self, id: &str) -> Option<Arc<Route<T>>> {
        let mut removed = self.no_matcher.remove(id);

        if removed.is_some() {
            self.count -= 1;

            return removed;
        }

        self.matchers.retain(|_, matcher| {
            if let Some(value) = matcher.remove(id) {
                removed = Some(value);
            }

            matcher.len() > 0
        });

        if removed.is_some() {
            self.count -= 1;
        }

        removed
    }

    pub fn match_request(&self, request: &Request) -> Vec<Arc<Route<T>>> {
        let mut routes = self.no_matcher.match_request(request);

        if let Some(remote_addr) = request.remote_addr.as_ref() {
            for (ip_cidr, matcher) in &self.matchers {
                if ip_cidr.match_ip(remote_addr) {
                    routes.extend(matcher.match_request(request));
                }
            }
        }

        routes
    }

    pub fn trace(&self, request: &Request) -> Vec<Trace<T>> {
        let mut traces = self.no_matcher.trace(request);

        if let Some(remote_addr) = request.remote_addr.as_ref() {
            for (ip_cidr, matcher) in &self.matchers {
                if ip_cidr.match_ip(remote_addr) {
                    let ip_traces = matcher.trace(request);

                    traces.push(Trace::new(
                        true,
                        true,
                        matcher.len() as u64,
                        ip_traces,
                        TraceInfo::Ip {
                            request: remote_addr.to_string(),
                            against: ip_cidr.to_string(),
                        },
                    ));
                } else {
                    traces.push(Trace::new(
                        false,
                        true,
                        matcher.len() as u64,
                        Vec::new(),
                        TraceInfo::Ip {
                            request: remote_addr.to_string(),
                            against: ip_cidr.to_string(),
                        },
                    ))
                }
            }
        }

        traces
    }

    pub fn cache(&mut self, limit: u64, level: u64) -> u64 {
        let mut new_limit = self.no_matcher.cache(limit, level);

        for matcher in self.matchers.values_mut() {
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
