mod config;
mod marker_string;
pub mod request_matcher;
mod route;
mod route_datetime;
mod route_header;
mod route_ip;
mod route_time;
mod route_weekday;
mod trace;
mod transformer;

use crate::http::Request;
pub use config::RouterConfig;
pub use marker_string::{Marker, MarkerString, StaticOrDynamic};
pub use request_matcher::{HostMatcher, IpMatcher, DateTimeMatcher, WeekdayMatcher, TimeMatcher, MethodMatcher, PathAndQueryMatcher, RequestMatcher, SchemeMatcher};
pub use route::{Route, RouteData};
pub use route_datetime::RouteDateTime;
pub use route_header::{RouteHeader, RouteHeaderKind};
pub use route_ip::RouteIp;
pub use route_time::RouteTime;
pub use route_weekday::RouteWeekday;
pub use trace::{RouteTrace, Trace};
pub use transformer::{Camelize, Dasherize, Lowercase, Replace, Slice, Transform, Underscorize, Uppercase};

use core::cmp::Reverse;

#[derive(Debug, Clone)]
pub struct Router<T: RouteData> {
    matcher: SchemeMatcher<T>,
    pub config: RouterConfig,
}

impl<T: RouteData> Default for Router<T> {
    fn default() -> Self {
        Router {
            matcher: SchemeMatcher::new(RouterConfig::default()),
            config: RouterConfig::default(),
        }
    }
}

impl<T: RouteData> Router<T> {
    pub fn from_config(config: RouterConfig) -> Self {
        Self {
            matcher: SchemeMatcher::new(config.clone()),
            config,
        }
    }

    pub fn insert(&mut self, route: Route<T>) {
        self.matcher.insert(route);
    }

    pub fn remove(&mut self, id: &str) -> bool {
        self.matcher.remove(id)
    }

    pub fn rebuild_request(&self, request: &Request) -> Request {
        Request::rebuild_with_config(&self.config, request)
    }

    pub fn match_request(&self, request: &Request) -> Vec<&Route<T>> {
        self.matcher.match_request(&request)
    }

    pub fn len(&self) -> usize {
        self.matcher.len()
    }

    pub fn is_empty(&self) -> bool {
        self.matcher.is_empty()
    }

    pub fn trace_request(&self, request: &Request) -> Vec<Trace<T>> {
        let request_rebuild = Request::rebuild_with_config(&self.config, request);

        self.matcher.trace(&request_rebuild)
    }

    pub fn get_route(&self, request: &Request) -> Option<&Route<T>> {
        let mut routes = self.match_request(request);

        if routes.is_empty() {
            return None;
        }

        routes.sort_by_key(|&b| Reverse(b.priority()));

        match routes.get(0) {
            None => None,
            Some(&route) => Some(route),
        }
    }

    pub fn get_trace(&self, request: &Request) -> RouteTrace<T> {
        let traces = self.trace_request(request);
        let mut routes_traces = Trace::get_routes_from_traces(&traces);
        let mut routes = Vec::new();

        for &route in &routes_traces {
            routes.push(route.clone());
        }

        routes_traces.sort_by(|&a, &b| b.priority().cmp(&a.priority()));

        let final_route = routes_traces.first().map(|&route| route.clone());

        RouteTrace::new(traces, routes, final_route)
    }

    pub fn cache(&mut self, limit: u64) {
        let mut prev_cache_limit = limit;
        let mut level = 0;

        while prev_cache_limit > 0 {
            let next_cache_limit = self.matcher.cache(prev_cache_limit, level);

            if next_cache_limit == prev_cache_limit && level > 5 {
                break;
            }

            level += 1;
            prev_cache_limit = next_cache_limit;
        }
    }
}
