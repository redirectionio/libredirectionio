use crate::router;
use crate::router::rule::{RouterTraceItem, Rule};
use std::collections::HashMap;
use std::error::Error;
use http::Request;

#[derive(Debug)]
pub struct RouterMethod {
    hosts_routers: HashMap<String, router::router_header::RouterHeader>,
    any_method_router: router::router_header::RouterHeader,
}

impl RouterMethod {
    pub fn new(rules: Vec<router::rule::Rule>) -> Result<RouterMethod, Box<dyn Error>> {
        let hosts_routers = HashMap::new();
        let any_method_router = router::router_header::RouterHeader::new(rules)?;

        Ok(RouterMethod{
            hosts_routers,
            any_method_router,
        })
    }
}

impl router::Router for RouterMethod {
    fn match_rule(&self, request: &Request<()>) -> Result<Vec<&Rule>, Box<dyn Error>> {
        self.any_method_router.match_rule(request)
    }

    fn trace(&self, request: &Request<()>) -> Result<Vec<RouterTraceItem>, Box<dyn Error>> {
        self.any_method_router.trace(request)
    }

    fn build_cache(&mut self, cache_limit: u64, level: u64) -> u64 {
        self.any_method_router.build_cache(cache_limit, level)
    }
}
