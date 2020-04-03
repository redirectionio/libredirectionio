use crate::router::rule;
use std::fmt::Debug;
use http::Request;

pub trait UrlMatcher: Debug + Send + Sync + 'static {
    fn match_rule(
        &self,
        request: &Request<()>,
        path: &str,
    ) -> Result<Vec<&rule::Rule>, Box<dyn std::error::Error>>;

    fn build_cache(&mut self, cache_limit: u64, level: u64) -> u64;

    fn trace(
        &self,
        request: &Request<()>,
        path: &str,
    ) -> Result<Vec<rule::RouterTraceItem>, Box<dyn std::error::Error>>;
}
