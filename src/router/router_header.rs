use crate::router;
use crate::router::rule::{RouterTraceItem, Rule};
use http::Request;
use std::error::Error;

#[derive(Debug)]
pub struct RouterHeader {
    any_header_router: router::router_path::RouterPath,
}

impl RouterHeader {
    pub fn new(rules: Vec<router::rule::Rule>) -> Result<RouterHeader, Box<dyn Error>> {
        let any_header_router = router::router_path::RouterPath::new(rules)?;

        Ok(RouterHeader {
            any_header_router,
        })
    }
}

impl router::Router for RouterHeader {
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
