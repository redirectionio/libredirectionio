use crate::router::rule;
use std::fmt::Debug;
use url::Url;

pub trait UrlMatcher: Debug + Send + 'static {
    fn match_rule(&self, url: &Url) -> Result<Vec<&rule::Rule>, Box<dyn std::error::Error>>;

    fn trace(&self, url: &Url) -> Result<Vec<rule::RouterTraceItem>, Box<dyn std::error::Error>>;

    fn get_rules(&self) -> Vec<&rule::Rule>;
}
