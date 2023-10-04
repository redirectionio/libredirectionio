mod config;
pub mod request_matcher;
mod route;
mod route_datetime;
mod route_header;
mod route_ip;
mod route_time;
mod route_weekday;
mod trace;

use crate::http::Request;
pub use config::RouterConfig;
pub use request_matcher::{DateTimeMatcher, HostMatcher, IpMatcher, MethodMatcher, PathAndQueryMatcher, SchemeMatcher};
pub use route::{IntoRoute, Route};
pub use route_datetime::RouteDateTime;
pub use route_header::{RouteHeader, RouteHeaderKind};
pub use route_ip::RouteIp;
pub use route_time::RouteTime;
pub use route_weekday::RouteWeekday;
pub use trace::{RouteTrace, Trace};

use core::cmp::Reverse;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Router<T> {
    matcher: SchemeMatcher<T>,
    pub config: Arc<RouterConfig>,
    pub routes: HashMap<String, Arc<Route<T>>>,
}

impl<T> Default for Router<T> {
    fn default() -> Self {
        let config = Arc::new(RouterConfig::default());

        Router {
            matcher: SchemeMatcher::new(config.clone()),
            config,
            routes: HashMap::new(),
        }
    }
}

impl<T> Router<T> {
    pub fn from_config(config: RouterConfig) -> Self {
        Self::from_arc_config(Arc::new(config))
    }

    pub fn from_arc_config(config: Arc<RouterConfig>) -> Self {
        Self {
            matcher: SchemeMatcher::new(config.clone()),
            config,
            routes: HashMap::new(),
        }
    }

    pub fn insert_route(&mut self, route: Route<T>) {
        let arc_route = Arc::new(route);

        self.matcher.insert(arc_route.clone());
        self.routes.insert(arc_route.id().to_string(), arc_route);
    }

    pub fn get_route_by_id(&self, id: &str) -> Option<Arc<Route<T>>> {
        self.routes.get(id).cloned()
    }

    pub fn remove(&mut self, id: &str) -> Option<Arc<Route<T>>> {
        if self.routes.contains_key(id) {
            self.routes.remove(id);

            self.matcher.remove(id)
        } else {
            None
        }
    }

    pub fn rebuild_request(&self, request: &Request) -> Request {
        Request::rebuild_with_config(self.config.as_ref(), request)
    }

    pub fn match_request(&self, request: &Request) -> Vec<Arc<Route<T>>> {
        self.matcher.match_request(request)
    }

    pub fn len(&self) -> usize {
        self.routes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.routes.is_empty()
    }

    pub fn routes(&self) -> &HashMap<String, Arc<Route<T>>> {
        &self.routes
    }

    pub fn trace_request(&self, request: &Request) -> Vec<Trace<T>> {
        let request_rebuild = Request::rebuild_with_config(self.config.as_ref(), request);

        self.matcher.trace(&request_rebuild)
    }

    pub fn get_route(&self, request: &Request) -> Option<Arc<Route<T>>> {
        let mut routes = self.match_request(request);

        if routes.is_empty() {
            return None;
        }

        routes.sort_by_key(|b| Reverse(b.priority()));
        routes.first().cloned()
    }

    pub fn get_trace(&self, request: &Request) -> RouteTrace<T> {
        let traces = self.trace_request(request);
        let mut routes_traces = Trace::get_routes_from_traces(&traces);
        let mut routes = Vec::new();

        for route in &routes_traces {
            routes.push(route.clone());
        }

        routes_traces.sort_by_key(|b| Reverse(b.priority()));

        let final_route = routes_traces.first().cloned();

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

impl<T> Router<T>
where
    T: IntoRoute<T>,
{
    pub fn insert(&mut self, item: T) {
        self.insert_route(item.into_route(self.config.as_ref()));
    }

    pub fn apply_change_set(&mut self, added: Vec<T>, updated: Vec<T>, removed: Vec<String>) {
        for id in removed {
            self.remove(id.as_str());
        }

        for item in updated {
            let route = item.into_route(self.config.as_ref());

            self.remove(route.id());
            self.insert_route(route);
        }

        for item in added {
            self.insert(item);
        }
    }
}
