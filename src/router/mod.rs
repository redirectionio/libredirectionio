pub mod api;
mod router_host;
mod router_path;
mod router_scheme;
pub mod rule;
mod url_matcher;
mod url_matcher_regex;
mod url_matcher_rules;

use crate::router::router_scheme::RouterScheme;
use regex::Regex;
use url::Url;

pub trait Router {
    fn match_rule(&self, url: Url) -> Vec<&rule::Rule>;
}

#[derive(Debug)]
pub struct MainRouter {
    router_scheme: router_scheme::RouterScheme,
}

impl MainRouter {
    pub fn new_from_data(data: String, cache: bool) -> MainRouter {
        let rules: Vec<rule::Rule> =
            serde_json::from_str(data.as_str()).expect("Cannot deserialize");
        let mut storage = Vec::new();

        for mut rule in rules {
            rule.compile(cache);

            storage.push(rule);
        }

        return MainRouter::new(storage, cache);
    }

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

    pub fn get_redirect(rule_to_redirect: &rule::Rule, url: String) -> String {
        let url_object = Url::parse(url.as_str()).expect("cannot parse url");
        let regex_groups = Regex::new(rule_to_redirect.regex_with_groups.as_ref().unwrap())
            .expect("cannot compile regex");

        let mut sorted_query = None;

        if url_object.query().is_some() {
            sorted_query = rule::build_sorted_query(url_object.query().unwrap().to_string());
        }

        let mut path = url_object.path().to_string();

        if sorted_query.is_some() {
            path.push_str(sorted_query.unwrap().as_str());
        }

        let capture_option = regex_groups.captures(path.as_str());

        if capture_option.is_none() {
            return "".to_string();
        }

        let capture_item = capture_option.unwrap();
        let target_opt = &rule_to_redirect.target;

        if target_opt.is_none() {
            return "".to_string();
        }

        let mut target = target_opt.as_ref().unwrap().to_string();

        for named_group in regex_groups.capture_names().into_iter() {
            if named_group.is_none() {
                continue;
            }

            let capture_match = capture_item.name(named_group.unwrap());

            if capture_match.is_none() {
                continue;
            }

            //@TODO Handle transformers

            target = target.replace(
                ["@", named_group.unwrap()].join("").as_str(),
                capture_match.unwrap().as_str(),
            );
        }

        return target;
    }

    pub fn has_match(&self, url: String) -> bool {
        return self.match_rules(url).len() > 0;
    }

    pub fn get_rules(&self) -> Vec<&rule::Rule> {
        return self.router_scheme.get_rules();
    }
}
