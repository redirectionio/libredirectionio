use crate::router;
use crate::router::router_method::RouterMethod;
use core::borrow::BorrowMut;
use std::collections::HashMap;
use http::Request;

#[derive(Debug)]
pub struct RouterHost {
    hosts_routers: HashMap<String, RouterMethod>,
    any_host_router: RouterMethod,
}

impl RouterHost {
    pub fn new(rules: Vec<router::rule::Rule>) -> Result<RouterHost, Box<dyn std::error::Error>> {
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

        let any_host_router = RouterMethod::new(any_host_rules)?;
        let mut hosts_routers = HashMap::new();

        for (host, rules_by_host) in hosts_router_rules {
            hosts_routers.insert(host.to_string(), RouterMethod::new(rules_by_host)?);
        }

        Ok(RouterHost {
            hosts_routers,
            any_host_router,
        })
    }
}

impl router::Router for RouterHost {
    fn match_rule(&self, request: &Request<()>) -> Result<Vec<&router::rule::Rule>, Box<dyn std::error::Error>> {
        if request.uri().host().is_some() {
            let host_str = request.uri().host().unwrap();

            if let Some(host_router) = self.hosts_routers.get(host_str) {
                let rules = host_router.match_rule(request)?;

                if !rules.is_empty() {
                    return Ok(rules);
                }
            }
        }

        self.any_host_router.match_rule(request)
    }

    fn trace(
        &self,
        request: &Request<()>,
    ) -> Result<Vec<router::rule::RouterTraceItem>, Box<dyn std::error::Error>> {
        let mut traces = Vec::new();

        if request.uri().host().is_some() && request.uri().host().unwrap() != "0.0.0.0" {
            let host_str = request.uri().host().unwrap().to_string();
            let has_host_router = self.hosts_routers.get(host_str.as_str());

            match has_host_router {
                Some(host_router) => {
                    traces.push(router::rule::RouterTraceItem {
                        rules_matches: Vec::new(),
                        rules_evaluated: Vec::new(),
                        matches: true,
                        prefix: request.uri().host().unwrap().to_string(),
                    });

                    traces.append(host_router.trace(request)?.borrow_mut());

                    return Ok(traces);
                }
                None => {
                    traces.push(router::rule::RouterTraceItem {
                        rules_matches: Vec::new(),
                        rules_evaluated: Vec::new(),
                        matches: false,
                        prefix: request.uri().host().unwrap().to_string(),
                    });
                }
            }
        }

        traces.push(router::rule::RouterTraceItem {
            rules_matches: Vec::new(),
            rules_evaluated: Vec::new(),
            matches: true,
            prefix: "any host router".to_string(),
        });

        traces.append(self.any_host_router.trace(request)?.borrow_mut());

        Ok(traces)
    }

    fn build_cache(&mut self, cache_limit: u64, level: u64) -> u64 {
        let mut new_cache_limit = cache_limit;

        new_cache_limit = self.any_host_router.build_cache(new_cache_limit, level);

        for router in self.hosts_routers.values_mut() {
            new_cache_limit = router.build_cache(new_cache_limit, level);
        }

        new_cache_limit
    }
}
