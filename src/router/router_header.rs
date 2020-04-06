use crate::router::Router;
use crate::router::rule::{RouterTraceItem, Rule};
use crate::router::router_path::RouterPath;
use http::Request;
use std::error::Error;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
pub struct RouterHeader {
    any_header_router: RouterPath,
}

impl RouterHeader {
    pub fn new(rules: Vec<Rule>) -> Result<RouterHeader, Box<dyn Error>> {
        let any_header_router = RouterPath::new(rules)?;

        Ok(RouterHeader {
            any_header_router,
        })
    }
}

fn most_used_header_key(rules: &Vec<Rule>, skipped_headers: &HashSet<String>) -> Option<String> {
    let mut headers_count = HashMap::new();
    let mut header_found = None;
    let mut header_found_count = 0;

    for rule in rules {
        if rule.source.headers.is_some() {
            for header in rule.source.headers.as_ref().unwrap() {
                let header_key = header.key.to_lowercase();

                if skipped_headers.contains(header_key.as_str()) {
                    continue;
                }

                let count = headers_count.get(header_key.as_str()).unwrap_or(&0).clone() + 1;

                if count > header_found_count {
                    header_found = Some(header_key.clone());
                    header_found_count = count;
                }

                headers_count.insert(header_key, count);
            }
        }
    }

    header_found
}

fn split_by_headers(mut rules: Vec<Rule>, mut skipped_headers: HashSet<String>) -> HashMap<Option<String>, Vec<Rule>> {
    let mut splitted = HashMap::new();
    let mut most_user_header_key = most_used_header_key(&rules, &skipped_headers);

    while most_user_header_key.is_some() {
        let header_key = most_user_header_key.unwrap();
        skipped_headers.insert(header_key.clone());

        let mut header_rules = Vec::new();
        let mut other_rules = Vec::new();

        for rule in rules {
            if rule.source.headers.is_none() {
                other_rules.push(rule);

                continue;
            }

            let mut has_header = false;

            for header in rule.source.headers.as_ref().unwrap() {
                if header.key.to_lowercase() == header_key {
                    has_header = true;
                    break;
                }
            }

            if has_header {
                header_rules.push(rule)
            } else {
                other_rules.push(rule)
            }
        }

        rules = other_rules;

        splitted.insert(Some(header_key), header_rules);
        most_user_header_key = most_used_header_key(&rules, &skipped_headers);

        if most_user_header_key.is_none() {
            splitted.insert(None, rules);

            break;
        }
    }

    splitted
}

impl Router for RouterHeader {
    fn match_rule(&self, request: &Request<()>) -> Result<Vec<&Rule>, Box<dyn Error>> {
        self.any_header_router.match_rule(request)
    }

    fn trace(&self, request: &Request<()>) -> Result<Vec<RouterTraceItem>, Box<dyn Error>> {
        self.any_header_router.trace(request)
    }

    fn build_cache(&mut self, cache_limit: u64, level: u64) -> u64 {
        self.any_header_router.build_cache(cache_limit, level)
    }
}
