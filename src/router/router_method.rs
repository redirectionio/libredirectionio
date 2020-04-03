use crate::router::Router;
use crate::router::router_header::RouterHeader;
use crate::router::rule::{RouterTraceItem, Rule};
use std::collections::HashMap;
use std::error::Error;
use http::Request;

#[derive(Debug)]
pub struct RouterMethod {
    methods_routers: HashMap<String, RouterHeader>,
    any_method_router: RouterHeader,
}

impl RouterMethod {
    pub fn new(rules: Vec<Rule>) -> Result<RouterMethod, Box<dyn Error>> {
        let mut methods_rules = HashMap::new();
        let mut any_method_rules = Vec::new();

        for rule in rules {
            if rule.source.methods.is_none() || rule.source.methods.as_ref().unwrap().is_empty() {
                any_method_rules.push(rule.clone());

                continue
            }

            for method in rule.source.methods.as_ref().unwrap() {
                if !methods_rules.contains_key(method.as_str()) {
                    methods_rules.insert(method.clone(), Vec::new());
                }

                methods_rules
                    .get_mut(method.as_str())
                    .unwrap()
                    .push(rule.clone());
            }
        }

        let mut methods_routers = HashMap::new();
        let any_method_router = RouterHeader::new(any_method_rules)?;

        for (method, rules_by_method) in methods_rules {
            methods_routers.insert(method.to_string(), RouterHeader::new(rules_by_method)?);
        }

        Ok(RouterMethod{
            methods_routers,
            any_method_router,
        })
    }
}

impl Router for RouterMethod {
    fn match_rule(&self, request: &Request<()>) -> Result<Vec<&Rule>, Box<dyn Error>> {
        let mut rules = self.any_method_router.match_rule(request)?;

        if self.methods_routers.contains_key(request.method().as_str()) {
            rules.extend(self.methods_routers.get(request.method().as_str()).unwrap().match_rule(request)?);
        }

        Ok(rules)
    }

    fn trace(&self, request: &Request<()>) -> Result<Vec<RouterTraceItem>, Box<dyn Error>> {
        let mut traces = self.any_method_router.trace(request)?;

        if self.methods_routers.contains_key(request.method().as_str()) {
            traces.extend(self.methods_routers.get(request.method().as_str()).unwrap().trace(request)?);
        }

        Ok(traces)
    }

    fn build_cache(&mut self, cache_limit: u64, level: u64) -> u64 {
        let mut new_cache_limit = cache_limit;

        new_cache_limit = self.any_method_router.build_cache(new_cache_limit, level);

        for router in self.methods_routers.values_mut() {
            new_cache_limit = router.build_cache(new_cache_limit, level);
        }

        new_cache_limit
    }
}
