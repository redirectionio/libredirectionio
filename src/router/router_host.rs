use crate::router;
use crate::router::router_path::RouterPath;
use core::borrow::BorrowMut;
use std::collections::HashMap;
use url::Url;

#[derive(Debug)]
pub struct RouterHost {
    hosts_routers: HashMap<String, router::router_path::RouterPath>,
    any_host_router: router::router_path::RouterPath,
}

impl RouterHost {
    pub fn new(
        rules: Vec<router::rule::Rule>,
        cache: bool,
    ) -> Result<RouterHost, Box<dyn std::error::Error>> {
        let mut hosts_router_rules = HashMap::new();
        let mut any_host_rules = Vec::new();

        for rule in rules {
            if rule.source.host.is_none() {
                any_host_rules.push(rule);

                continue;
            }

            let host = rule.source.host.as_ref().unwrap();

            if host == "" {
                any_host_rules.push(rule);

                continue;
            }

            if !hosts_router_rules.contains_key(host.as_str()) {
                hosts_router_rules.insert(host.clone(), Vec::new());
            }

            hosts_router_rules
                .get_mut(host.as_str())
                .unwrap()
                .push(rule);
        }

        let any_host_router = RouterPath::new(any_host_rules, cache)?;
        let mut hosts_routers = HashMap::new();

        for (host, rules_by_host) in hosts_router_rules {
            hosts_routers.insert(host.to_string(), RouterPath::new(rules_by_host, cache)?);
        }

        return Ok(RouterHost {
            hosts_routers,
            any_host_router,
        });
    }
}

impl router::Router for RouterHost {
    fn match_rule(&self, url: Url) -> Result<Vec<&router::rule::Rule>, Box<dyn std::error::Error>> {
        if url.host().is_some() {
            let host_str = url.host().unwrap().to_string();
            let host_router = self.hosts_routers.get(host_str.as_str());

            if host_router.is_some() {
                let rule = host_router.unwrap().match_rule(url.clone());

                if rule.is_err() {
                    return rule;
                }

                if rule.as_ref().unwrap().len() > 0 {
                    return rule;
                }
            }
        }

        return self.any_host_router.match_rule(url);
    }

    fn trace(
        &self,
        url: Url,
    ) -> Result<Vec<router::rule::RouterTraceItem>, Box<dyn std::error::Error>> {
        let mut traces = Vec::new();

        if url.host().is_some() && url.host().unwrap().to_string() != "0.0.0.0".to_string() {
            let host_str = url.host().unwrap().to_string();
            let host_router = self.hosts_routers.get(host_str.as_str());

            if host_router.is_some() {
                traces.push(router::rule::RouterTraceItem {
                    rules_matches: Vec::new(),
                    rules_evaluated: Vec::new(),
                    matches: true,
                    prefix: url.host().unwrap().to_string(),
                });

                traces.append(host_router.unwrap().trace(url)?.borrow_mut());

                return Ok(traces);
            } else {
                traces.push(router::rule::RouterTraceItem {
                    rules_matches: Vec::new(),
                    rules_evaluated: Vec::new(),
                    matches: false,
                    prefix: url.host().unwrap().to_string(),
                });
            }
        }

        traces.push(router::rule::RouterTraceItem {
            rules_matches: Vec::new(),
            rules_evaluated: Vec::new(),
            matches: true,
            prefix: "any host router".to_string(),
        });

        traces.append(self.any_host_router.trace(url)?.borrow_mut());

        return Ok(traces);
    }
}
