use crate::api::{RouterTrace, Rule};
use crate::http::Request;
use crate::router::Router;
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Impact {
    examples: Vec<ImpactExample>,
    change_set: ChangeSet,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImpactExample {
    url: String,
    method: Option<String>,
    headers: Option<Vec<ImpactExampleHeader>>,
    ip_address: Option<String>,
    response_status_code: Option<u16>,
    must_match: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImpactExampleHeader {
    name: String,
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImpactResultItem {
    example: ImpactExample,
    trace_unique: RouterTrace,
    trace_before_update: RouterTrace,
    trace_after_update: RouterTrace,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChangeSet {
    new: Vec<Rule>,
    update: Vec<Rule>,
    delete: Vec<String>,
}

impl Impact {
    pub fn create_result(router: &Router<Rule>, impact: &Impact) -> Vec<ImpactResultItem> {
        let mut trace_router: Router<Rule> = Router::from_config(router.config.clone());
        let mut next_router = router.clone();

        // Remove rules
        for id in &impact.change_set.delete {
            next_router.remove(id.as_str());
        }

        // Update rules
        for rule in &impact.change_set.update {
            next_router.remove(rule.id.as_str());
        }

        for rule in &impact.change_set.update {
            next_router.insert(rule.clone().into_route(&next_router.config));
            trace_router.insert(rule.clone().into_route(&trace_router.config));
        }

        // Add rules
        for rule in &impact.change_set.new {
            next_router.insert(rule.clone().into_route(&next_router.config));
            trace_router.insert(rule.clone().into_route(&trace_router.config));
        }

        let mut items = Vec::new();

        for example in &impact.examples {
            let mut builder = http::Request::<()>::builder()
                .uri(example.url.as_str())
                .method(match &example.method {
                    None => "GET",
                    Some(method) => method.as_str(),
                });

            if example.headers.is_some() {
                for header in example.headers.as_ref().unwrap() {
                    builder = builder.header(header.name.as_str(), header.value.clone());
                }
            }

            let http_request_res = builder.body(());

            if http_request_res.is_err() {
                continue;
            }

            let http_request = http_request_res.unwrap();
            let path_and_query = match http_request.uri().path_and_query() {
                None => "",
                Some(path_and_query) => path_and_query.as_str(),
            };
            let ip_address = match &example.ip_address {
                Some(ip) => IpAddr::from_str(ip.as_str()).ok(),
                None => None,
            };

            let mut request = Request::from_config(
                &router.config,
                path_and_query.to_string(),
                http_request.uri().host().map(|s| s.to_string()),
                http_request.uri().scheme_str().map(|s| s.to_string()),
                example.method.clone(),
                ip_address,
                Some(true),
            );

            if example.headers.is_some() {
                for header in example.headers.as_ref().unwrap() {
                    request.add_header(header.name.clone(), header.value.clone(), router.config.ignore_header_case);
                }
            }

            let trace_before_update = RouterTrace::create_from_router(router, &request);
            let trace_after_update = RouterTrace::create_from_router(&next_router, &request);
            let trace_unique = RouterTrace::create_from_router(&trace_router, &request);

            items.push(ImpactResultItem {
                example: example.clone(),
                trace_before_update,
                trace_after_update,
                trace_unique,
            });
        }

        items
    }
}
