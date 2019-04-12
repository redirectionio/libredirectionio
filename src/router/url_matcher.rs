use crate::router::rule;
use std::fmt::Debug;
use url::Url;

pub trait UrlMatcher: Debug + 'static {
    fn match_rule(&self, url: &Url) -> Vec<&rule::Rule>;

    fn get_rules(&self) -> Vec<&rule::Rule>;
}
