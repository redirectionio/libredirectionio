mod datetime;
mod header;
mod host;
mod ip;
mod matcher_tree_storage;
mod method;
mod path_and_query;
mod route_matcher;
mod scheme;
mod time;
mod weekday;

use crate::http::Request;
use crate::router::{Route, RouteData, Trace};
use std::fmt::Debug;

pub use datetime::DateTimeMatcher;
pub use header::{HeaderMatcher, ValueCondition as HeaderValueCondition};
pub use host::HostMatcher;
pub use ip::IpMatcher;
pub use method::MethodMatcher;
pub use path_and_query::PathAndQueryMatcher;
pub use route_matcher::RouteMatcher;
pub use scheme::SchemeMatcher;
pub use time::TimeMatcher;
pub use weekday::WeekdayMatcher;

pub trait RequestMatcher<T: RouteData>: Debug + Send + Sync {
    fn insert(&mut self, route: Route<T>);

    fn remove(&mut self, id: &str) -> bool;

    fn match_request(&self, request: &Request) -> Vec<&Route<T>>;

    fn trace(&self, request: &Request) -> Vec<Trace<T>>;

    fn cache(&mut self, limit: u64, level: u64) -> u64;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool;

    fn box_clone(&self) -> Box<dyn RequestMatcher<T>>;
}

impl<T: RouteData> Clone for Box<dyn RequestMatcher<T>> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}
