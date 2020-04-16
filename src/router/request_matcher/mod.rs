mod header;
mod host;
mod method;
mod path_and_query;
mod scheme;
mod regex_item_matcher;
mod route_matcher;

use http::Request;
use std::fmt::Debug;
use crate::router::{Route, RouteData, Trace};

pub use scheme::SchemeMatcher;
pub use host::HostMatcher;
pub use method::MethodMatcher;
pub use header::HeaderMatcher;
pub use path_and_query::PathAndQueryMatcher;
pub use route_matcher::RouteMatcher;

pub trait RequestMatcher<T>: Debug + Send + Sync where T: RouteData {
    fn insert(&mut self, route: Route<T>);

    fn remove(&mut self, id: &str) -> Vec<Route<T>>;

    fn match_request(&self, request: &Request<()>) -> Vec<&Route<T>>;

    fn trace(&self, request: &Request<()>) -> Vec<Trace>;

    fn cache(&mut self, limit: u64, level: u64) -> u64;

    fn len(&self) -> usize;
}
