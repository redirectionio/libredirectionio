mod api;
mod router_host;
mod router_path;
mod router_scheme;
mod rule;
mod url_matcher;
mod url_matcher_regex;
mod url_matcher_rules;

use crate::router::router_scheme::RouterScheme;
use std::fs;
use url::Url;

pub trait Router {
    fn match_rule(&self, url: Url) -> Vec<&rule::Rule>;
}

#[derive(Debug)]
pub struct MainRouter {
    router_scheme: router_scheme::RouterScheme,
}

impl MainRouter {
    pub fn new(rules: Vec<rule::Rule>, cache: bool) -> MainRouter {
        let router_scheme = RouterScheme::new(rules, cache);

        MainRouter { router_scheme }
    }

    pub fn match_rules(&self, url: String) -> Vec<&rule::Rule> {
        let url_object = Url::parse(url.as_str()).expect("cannot parse url");

        return self.router_scheme.match_rule(url_object);
    }

    pub fn match_rule(&self, url: String) -> Option<&rule::Rule> {
        let mut rules = self.match_rules(url);

        if rules.len() == 0 {
            return None;
        }

        rules.sort_by(|a, b| a.rank.cmp(&b.rank));

        return Some(*rules.first().unwrap());
    }

    pub fn has_match(&self, url: String) -> bool {
        return self.match_rules(url).len() > 0;
    }

    pub fn get_rules(&self) -> Vec<&rule::Rule> {
        return self.router_scheme.get_rules();
    }
}

pub fn create_test_router(cache: bool) -> MainRouter {
    let data = fs::read_to_string(
        "/home/joelwurtz/Archive/Redirection/redirection.io/clients/libredirectionio/rules.json",
    )
    .expect("Unable to read file");
    let deserialized: api::ApiAgentRuleResponse = serde_json::from_str(&data).unwrap();
    let mut storage = Vec::new();

    for mut rule in deserialized.rules {
        rule.compile(cache);

        storage.push(rule);
    }

    MainRouter::new(storage, cache)
}
