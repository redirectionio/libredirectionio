pub mod api;
mod router_host;
mod router_path;
mod router_scheme;
mod router_header;
mod router_method;
pub mod rule;
mod transform;
mod url_matcher;
mod url_matcher_regex;
mod url_matcher_rules;

use crate::router::router_scheme::RouterScheme;
use crate::router::rule::build_sorted_query;
use regex::Regex;
use std::time;
use url;
use url::Url;

pub trait Router {
    fn match_rule(&self, url: Url) -> Result<Vec<&rule::Rule>, Box<dyn std::error::Error>>;
    fn trace(&self, url: Url) -> Result<Vec<rule::RouterTraceItem>, Box<dyn std::error::Error>>;
    fn build_cache(&mut self, cache_limit: u64, level: u64) -> u64;
}

#[derive(Debug)]
pub struct MainRouter {
    router_scheme: router_scheme::RouterScheme,
}

impl MainRouter {
    pub fn new_from_data(
        data: String,
        cache_limit: u64,
    ) -> Result<MainRouter, Box<dyn std::error::Error>> {
        let rules: Vec<rule::Rule> = serde_json::from_str(data.as_str())?;
        let mut storage = Vec::new();

        for mut rule in rules {
            let compile_result = rule.compile(false);

            if compile_result.is_err() {
                error!(
                    "Skipping rule {}, compilation failed: {}",
                    rule.id,
                    compile_result.err().unwrap()
                );
            } else {
                storage.push(rule);
            }
        }

        let mut router = MainRouter::new(storage)?;
        router.build_cache(cache_limit);

        Ok(router)
    }

    pub fn new(rules: Vec<rule::Rule>) -> Result<MainRouter, Box<dyn std::error::Error>> {
        let router_scheme = RouterScheme::new(rules)?;

        Ok(MainRouter { router_scheme })
    }

    fn build_cache(&mut self, cache_limit: u64) {
        let mut prev_cache_limit = cache_limit;
        let mut level = 0;

        while prev_cache_limit > 0 {
            let next_cache_limit = self.router_scheme.build_cache(prev_cache_limit, level);

            if next_cache_limit == prev_cache_limit {
                break;
            }

            level += 1;
            prev_cache_limit = next_cache_limit;
        }
    }

    fn parse_url(url_str: String) -> Result<Url, url::ParseError> {
        let options = url::Url::options();
        let base_url = Url::parse("scheme://0.0.0.0")?;
        let parser = options.base_url(Some(&base_url));

        let url_obj = parser.parse(url_str.as_str())?;

        Ok(MainRouter::sort_query(url_obj))
    }

    fn sort_query(url_obj: Url) -> Url {
        if url_obj.query().is_none() {
            return url_obj;
        }

        let mut new_url_obj = url_obj;
        let query_string = build_sorted_query(new_url_obj.query().unwrap().to_string());

        match query_string {
            Some(query) => new_url_obj.set_query(Some(query.as_str())),
            None => {
                new_url_obj.set_query(None);
            }
        }

        new_url_obj
    }

    pub fn match_rules(
        &self,
        url_str: String,
    ) -> Result<Vec<&rule::Rule>, Box<dyn std::error::Error>> {
        let url_object = MainRouter::parse_url(url_str)?;

        self.router_scheme.match_rule(url_object)
    }

    pub fn match_rule(
        &self,
        url: String,
    ) -> Result<Option<&rule::Rule>, Box<dyn std::error::Error>> {
        let mut rules = self.match_rules(url)?;

        if rules.is_empty() {
            return Ok(None);
        }

        rules.sort_by(|a, b| a.rank.cmp(&b.rank));

        Ok(Some(*rules.first().unwrap()))
    }

    pub fn trace(&self, url_str: String) -> Result<rule::RouterTrace, Box<dyn std::error::Error>> {
        let url_object = MainRouter::parse_url(url_str.clone())?;
        let traces = self.router_scheme.trace(url_object.clone())?;
        let start = time::Instant::now();
        let mut matched_rules = self.router_scheme.match_rule(url_object.clone())?;
        let elapsed = (start.elapsed().as_micros() as f64) / 1000.0;
        let mut final_rule = None;

        if !matched_rules.is_empty() {
            matched_rules.sort_by(|a, b| a.rank.cmp(&b.rank));
            final_rule = Some((*matched_rules.first().unwrap()).clone());
        }

        let mut rules = Vec::new();

        for matched_rule in matched_rules {
            rules.push(matched_rule.clone());
        }

        let mut redirect = None;

        if final_rule.is_some() {
            let target = MainRouter::get_redirect(final_rule.as_ref().unwrap(), url_str.clone())?;

            redirect = Some(rule::Redirect {
                status: final_rule.as_ref().unwrap().redirect_code,
                target,
            });
        }

        let trace = rule::RouterTrace {
            final_rule,
            traces,
            rules,
            response: redirect,
            duration: elapsed,
        };

        Ok(trace)
    }

    pub fn get_redirect(
        rule_to_redirect: &rule::Rule,
        url_str: String,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let url_object = MainRouter::parse_url(url_str)?;

        // No markers
        if rule_to_redirect.static_path.is_some() {
            return Ok(rule_to_redirect.target.as_ref().unwrap().clone());
        }

        let regex_groups_str = [
            "^",
            rule_to_redirect.regex_with_groups.as_ref().unwrap(),
            "$",
        ]
        .join("");
        let regex_groups = Regex::new(regex_groups_str.as_str())?;

        let mut path = url_object.path().to_string();

        if url_object.query().is_some() {
            let sorted_query = rule::build_sorted_query(url_object.query().unwrap().to_string());

            if sorted_query.is_some() {
                path = [path.as_str(), "?", sorted_query.as_ref().unwrap().as_str()].join("");
            }
        }

        let target_opt = rule_to_redirect.target.as_ref();

        if target_opt.is_none() {
            return Ok("".to_string());
        }

        let mut target = target_opt.as_ref().unwrap().to_string();
        let mut capture_option = regex_groups.captures(path.as_str());

        if capture_option.is_none() {
            capture_option = regex_groups.captures(path.as_str());

            if capture_option.is_none() {
                return Ok(target);
            }
        }

        let capture_item = capture_option.unwrap();

        if target_opt.is_none() {
            return Ok(target);
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

        Ok(target)
    }
}
