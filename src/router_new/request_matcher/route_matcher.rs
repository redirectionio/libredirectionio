use super::super::trace::TraceInfo;
use super::super::{Route, Trace};
use crate::http::Request;

#[derive(Debug, Clone)]
pub struct RouteMatcher<T> {
    routes: Vec<Route<T>>,
}

impl<T> RouteMatcher<T> {
    pub fn insert(&mut self, route: Route<T>) {
        self.routes.push(route)
    }

    pub fn remove(&mut self, id: &str) -> bool {
        let mut i = 0;
        let mut removed = false;

        while i != self.routes.len() {
            let item = &mut self.routes[i];

            if item.id() == id {
                self.routes.remove(i);

                removed = true;
            } else {
                i += 1;
            }
        }

        removed
    }

    pub fn match_request(&self, _request: &Request) -> Vec<&Route<T>> {
        self.routes.iter().collect::<Vec<_>>()
    }

    pub fn trace(&self, _request: &Request) -> Vec<Trace<T>> {
        let traces = vec![Trace::new(
            true,
            true,
            self.routes.len() as u64,
            Vec::new(),
            TraceInfo::Storage {
                routes: self.routes.clone(),
            },
        )];

        traces
    }

    pub fn cache(&mut self, limit: u64, _level: u64) -> u64 {
        limit
    }

    pub fn len(&self) -> usize {
        self.routes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.routes.is_empty()
    }
}

impl<T> Default for RouteMatcher<T> {
    fn default() -> RouteMatcher<T> {
        RouteMatcher { routes: Vec::new() }
    }
}
