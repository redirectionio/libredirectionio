use crate::router::request_matcher::RequestMatcher;
use crate::router::{Route, RouteData, Trace};
use http::Request;

#[derive(Debug, Clone)]
pub struct RouteMatcher<T> where T: RouteData {
    routes: Vec<Route<T>>,
}

impl<T: RouteData> RequestMatcher<T> for RouteMatcher<T> {
    fn insert(&mut self, route: Route<T>) {
        self.routes.push(route)
    }

    fn remove(&mut self, id: &str) -> Vec<Route<T>> {
        let mut i = 0;
        let mut removed = Vec::new();

        while i != self.routes.len() {
            let item = &mut self.routes[i];

            if item.id() == id {
                removed.push(self.routes.remove(i));
            } else {
                i += 1;
            }
        }

        removed
    }

    fn match_request(&self, _request: &Request<()>) -> Vec<&Route<T>> {
        self.routes.iter().collect::<Vec<_>>()
    }

    fn trace(&self, _request: &Request<()>) -> Vec<Trace<T>> {
        let traces = vec![
            Trace::new(
                "route_matcher".to_string(),
                true,
                true,
                self.routes.len() as u64,
                Vec::new(),
                self.routes.clone(),
            )
        ];

        traces
    }

    fn cache(&mut self, limit: u64, _level: u64) -> u64 {
        limit
    }

    fn len(&self) -> usize {
        self.routes.len()
    }

    fn is_empty(&self) -> bool {
        self.routes.is_empty()
    }

    fn box_clone(&self) -> Box<dyn RequestMatcher<T>> {
        Box::new((*self).clone())
    }
}

impl<T: RouteData> Default for RouteMatcher<T> {
    fn default() -> RouteMatcher<T> {
        RouteMatcher {
            routes: Vec::new(),
        }
    }
}
