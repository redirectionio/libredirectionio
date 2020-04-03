use crate::router;
use std::collections::HashMap;

pub struct RouterMethod {
    hosts_routers: HashMap<String, router::router_header::RouterHeader>,
    any_method_router: router::router_header::RouterHeader,
}

impl RouterMethod {
    pub fn new(rules: Vec<router::rule::Rule>) -> Result<RouterMethod, Box<dyn std::error::Error>> {
        let hosts_routers = HashMap::new();
        let any_method_router = router::router_header::RouterHeader::new(rules)?;

        Ok(RouterMethod{
            hosts_routers,
            any_method_router,
        })
    }
}
