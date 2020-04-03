use crate::router;

pub struct RouterHeader {
    any_header_router: router::router_path::RouterPath,
}

impl RouterHeader {
    pub fn new(rules: Vec<router::rule::Rule>) -> Result<RouterHeader, Box<dyn std::error::Error>> {
        let any_header_router = router::router_path::RouterPath::new(rules)?;

        Ok(RouterHeader {
            any_header_router,
        })
    }
}
