pub mod api;
mod router_host;
mod router_path;
mod router_scheme;
pub mod rule;
mod transform;
mod url_matcher;
mod url_matcher_regex;
mod url_matcher_rules;

use crate::router::router_scheme::RouterScheme;
use regex::Regex;
use std::collections::btree_map::BTreeMap;
use std::time;
use url;
use url::Url;

pub trait Router {
    fn match_rule(&self, url: Url) -> Vec<&rule::Rule>;
    fn trace(&self, url: Url) -> Vec<rule::RouterTraceItem>;
}

#[derive(Debug)]
pub struct MainRouter {
    router_scheme: router_scheme::RouterScheme,
}

impl MainRouter {
    pub fn new_from_data(data: String, cache: bool) -> MainRouter {
        let rules: Vec<rule::Rule> =
            serde_json::from_str(data.as_str()).expect("Cannot deserialize rules list");
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

    fn parse_url(url_str: String) -> Url {
        let url_str_decoded = url_str.clone();
        let url_result = Url::parse(url_str_decoded.as_str());

        if url_result.is_err() {
            let error = url_result.as_ref().unwrap_err();

            if *error == url::ParseError::RelativeUrlWithoutBase {
                let options = url::Url::options();
                let base_url = Url::parse("https://www.test.com").expect("Cannot parse base url");
                let parser = options.base_url(Some(&base_url));

                let mut url_obj = parser
                    .parse(url_str_decoded.as_str())
                    .expect("cannot parse url");

                return MainRouter::sort_query(url_obj);
            }
        }

        return url_result.expect("cannot parse url");
    }

    fn sort_query(url_obj: Url) -> Url {
        let mut new_url_obj = url_obj;
        let hash_query: BTreeMap<_, _> = new_url_obj.query_pairs().into_owned().collect();

        let mut query_string = "".to_string();

        for (key, value) in &hash_query {
            query_string.push_str(key);
            query_string.push_str("=");
            query_string.push_str(value);
            query_string.push_str("&");
        }

        query_string.pop();

        if !query_string.is_empty() {
            new_url_obj.set_query(Some(query_string.as_str()))
        } else {
            new_url_obj.set_query(None);
        }

        return new_url_obj;
    }

    pub fn match_rules(&self, url_str: String) -> Vec<&rule::Rule> {
        let url_object = MainRouter::parse_url(url_str);

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

    pub fn trace(&self, url_str: String) -> rule::RouterTrace {
        let url_object = MainRouter::parse_url(url_str.clone());
        let traces = self.router_scheme.trace(url_object.clone());
        let start = time::Instant::now();
        let mut matched_rules = self.router_scheme.match_rule(url_object.clone());
        let elapsed = start.elapsed().as_millis();
        let mut final_rule = None;

        if matched_rules.len() > 0 {
            matched_rules.sort_by(|a, b| a.rank.cmp(&b.rank));
            final_rule = Some((*matched_rules.first().unwrap()).clone());
        }

        let mut rules = Vec::new();

        for matched_rule in matched_rules {
            rules.push(matched_rule.clone());
        }

        let mut redirect = None;

        if final_rule.is_some() {
            let target = MainRouter::get_redirect(final_rule.as_ref().unwrap(), url_str.clone());

            redirect = Some(rule::Redirect {
                status: final_rule.as_ref().unwrap().redirect_code,
                target,
            });
        }

        return rule::RouterTrace {
            final_rule,
            traces,
            rules,
            response: redirect,
            duration: elapsed,
        };
    }

    pub fn get_redirect(rule_to_redirect: &rule::Rule, url_str: String) -> String {
        let url_object = MainRouter::parse_url(url_str);
        let regex_groups_str = [
            "^",
            rule_to_redirect.regex_with_groups.as_ref().unwrap(),
            "$",
        ]
        .join("");
        let regex_groups = Regex::new(regex_groups_str.as_str()).expect("cannot compile regex");

        let mut path = url_object.path().to_string();
        let mut path_decoded = url::percent_encoding::percent_decode(path.as_bytes())
            .decode_utf8()
            .expect("cannot create utf8 path")
            .to_string();

        if url_object.query().is_some() {
            let sorted_query = rule::build_sorted_query(url_object.query().unwrap().to_string());

            if sorted_query.is_some() {
                path = [path.as_str(), "?", sorted_query.as_ref().unwrap().as_str()].join("");
                path_decoded = [
                    path_decoded.as_str(),
                    "?",
                    sorted_query.as_ref().unwrap().as_str(),
                ]
                .join("");
            }
        }

        let target_opt = rule_to_redirect.target.as_ref();

        if target_opt.is_none() {
            return "".to_string();
        }

        let mut target = target_opt.as_ref().unwrap().to_string();
        let mut capture_option = regex_groups.captures(path.as_str());

        if capture_option.is_none() {
            capture_option = regex_groups.captures(path_decoded.as_str());

            if capture_option.is_none() {
                return target;
            }
        }

        let capture_item = capture_option.unwrap();

        if target_opt.is_none() {
            return target;
        }

        for named_group in regex_groups.capture_names().into_iter() {
            if named_group.is_none() {
                continue;
            }

            let capture_match = capture_item.name(named_group.unwrap());

            if capture_match.is_none() {
                continue;
            }

            let mut marker_data = capture_match.unwrap().as_str().to_string();
            let marker_name = named_group.unwrap().to_string();

            if rule_to_redirect.markers.is_some() {
                for marker in rule_to_redirect.markers.as_ref().unwrap() {
                    if marker.name == marker_name && marker.transformers.is_some() {
                        for transformer in marker.transformers.as_ref().unwrap() {
                            marker_data = transform::transform(marker_data, transformer);
                        }
                    }
                }
            }

            target = target.replace(
                ["@", marker_name.as_str()].join("").as_str(),
                marker_data.as_str(),
            );
        }

        return target;
    }
}
