use crate::router;
use crate::router::router_path::RouterPath;
use std::collections::HashMap;
use url::Url;

#[derive(Debug)]
pub struct RouterHost {
    hosts_routers: HashMap<String, router::router_path::RouterPath>,
    any_host_router: router::router_path::RouterPath,
}

impl RouterHost {
    pub fn new(rules: Vec<router::rule::Rule>, cache: bool) -> RouterHost {
        let mut hosts_router_rules = HashMap::new();
        let mut any_host_rules = Vec::new();

        for rule in rules {
            if rule.source.host.is_none() {
                any_host_rules.push(rule);

                break;
            }

            let host = rule.source.host.as_ref().unwrap();

            if host == "" {
                any_host_rules.push(rule);

                break;
            }

            if !hosts_router_rules.contains_key(host.as_str()) {
                hosts_router_rules.insert(host.clone(), Vec::new());
            }

            hosts_router_rules
                .get_mut(host.as_str())
                .unwrap()
                .push(rule);
        }

        let any_host_router = RouterPath::new(any_host_rules, cache);
        let mut hosts_routers = HashMap::new();

        for (host, rules_by_host) in hosts_router_rules {
            hosts_routers.insert(host.to_string(), RouterPath::new(rules_by_host, cache));
        }

        return RouterHost {
            hosts_routers,
            any_host_router,
        };
    }

    pub fn get_rules(&self) -> Vec<&router::rule::Rule> {
        let mut rules = Vec::new();

        rules.append(&mut self.any_host_router.get_rules());

        for (_, router) in &self.hosts_routers {
            rules.append(&mut router.get_rules());
        }

        return rules;
    }
}

impl router::Router for RouterHost {
    fn match_rule(&self, url: Url) -> Vec<&router::rule::Rule> {
        if url.host().is_some() {
            let host_str = url.host().unwrap().to_string();
            let host_router = self.hosts_routers.get(host_str.as_str());

            if host_router.is_some() {
                return host_router.unwrap().match_rule(url);
            }
        }

        return self.any_host_router.match_rule(url);
    }
}
