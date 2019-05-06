use crate::router;
use core::borrow::BorrowMut;
use url::Url;

#[derive(Debug)]
pub struct RouterScheme {
    http_router: router::router_host::RouterHost,
    https_router: router::router_host::RouterHost,
    any_scheme_router: router::router_host::RouterHost,
}

impl RouterScheme {
    pub fn new(rules: Vec<router::rule::Rule>, cache: bool) -> RouterScheme {
        let mut http_rules = Vec::new();
        let mut https_rules = Vec::new();
        let mut any_scheme_rules = Vec::new();

        for rule in rules {
            let scheme = rule.source.scheme.clone();

            match scheme {
                None => {
                    any_scheme_rules.push(rule);
                }
                Some(string) => match string.as_str() {
                    "https" => https_rules.push(rule),
                    "http" => http_rules.push(rule),
                    _ => any_scheme_rules.push(rule),
                },
            }
        }

        return RouterScheme {
            http_router: router::router_host::RouterHost::new(http_rules, cache),
            https_router: router::router_host::RouterHost::new(https_rules, cache),
            any_scheme_router: router::router_host::RouterHost::new(any_scheme_rules, cache),
        };
    }
}

impl router::Router for RouterScheme {
    fn match_rule(&self, url: Url) -> Vec<&router::rule::Rule> {
        let mut rules_found = Vec::new();

        rules_found.append(self.any_scheme_router.match_rule(url.clone()).borrow_mut());

        if url.scheme() == "http" {
            rules_found.append(self.http_router.match_rule(url.clone()).borrow_mut());
        }

        if url.scheme() == "https" {
            rules_found.append(self.https_router.match_rule(url.clone()).borrow_mut());
        }

        return rules_found;
    }

    fn trace(&self, url: Url) -> Vec<router::rule::RouterTraceItem> {
        let mut traces = self.any_scheme_router.trace(url.clone());

        if url.scheme() == "http" {
            traces.append(self.http_router.trace(url.clone()).borrow_mut());
        }

        if url.scheme() == "https" {
            traces.append(self.https_router.trace(url.clone()).borrow_mut());
        }

        return traces;
    }
}
