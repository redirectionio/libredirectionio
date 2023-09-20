use crate::http::Request;
use super::super::route_ip::RouteIp;
use super::super::trace::TraceInfo;
use super::super::{MethodMatcher, Route, Trace};
use std::collections::HashMap;
use std::sync::Arc;
use crate::router::RouterConfig;

#[derive(Debug, Clone)]
pub struct IpMatcher<T> {
    matchers: HashMap<RouteIp, MethodMatcher<T>>,
    no_matcher: MethodMatcher<T>,
    count: usize,
    config: Arc<RouterConfig>,
}

impl<T> IpMatcher<T> {
    fn insert(&mut self, route: Route<T>) {
        self.count += 1;

        match route.ips() {
            Some(ips) => {
                for ip in ips {
                    self.matchers
                        .entry(ip.clone())
                        .or_insert_with(|| MethodMatcher::new(self.config.clone())
                        .insert(route.clone());
                }
            }
            None => {
                self.no_matcher.insert(route);
            }
        }
    }

    fn remove(&mut self, id: &str) -> bool {
        let mut removed = false;

        if self.no_matcher.remove(id) {
            self.count -= 1;

            return true;
        }

        self.matchers.retain(|_, matcher| {
            removed = removed || matcher.remove(id);

            matcher.len() > 0
        });

        if removed {
            self.count -= 1;
        }

        removed
    }

    fn match_request(&self, request: &Request) -> Vec<&Route<T>> {
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

    fn trace(&self, request: &Request) -> Vec<Trace<T>> {
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

    fn cache(&mut self, limit: u64, level: u64) -> u64 {
        let mut new_limit = self.no_matcher.cache(limit, level);

        for matcher in self.matchers.values_mut() {
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

    pub fn new(config: Arc<RouterConfig>) -> Self {
        IpMatcher {
            matchers: HashMap::new(),
            no_matcher: Self::create_sub_matcher(),
            count: 0,
            config,
        }
    }
}

impl<T: RouteData> Default for IpMatcher<T> {
    fn default() -> Self {
        IpMatcher {
            matchers: HashMap::new(),
            no_matcher: Self::create_sub_matcher(),
            count: 0,
        }
    }
}

impl<T: RouteData> IpMatcher<T> {
    pub fn create_sub_matcher() -> Box<dyn RequestMatcher<T>> {
        Box::<MethodMatcher<T>>::default()
    }
}
