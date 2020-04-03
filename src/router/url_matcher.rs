use crate::router::rule;
use std::fmt::Debug;
use url::Url;

pub trait UrlMatcher: Debug + Send + Sync + 'static {
    fn match_rule(
        &self,
        url: &Url,
        path: &str,
    ) -> Result<Vec<&rule::Rule>, Box<dyn std::error::Error>>;

    fn build_cache(&mut self, cache_limit: u64, level: u64) -> u64;

    fn trace(
        &self,
        url: &Url,
        path: &str,
    ) -> Result<Vec<rule::RouterTraceItem>, Box<dyn std::error::Error>>;
}
