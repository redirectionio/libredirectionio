use crate::api::{RouterTrace, Rule};
use crate::http::{Request, STATIC_QUERY_PARAM_SKIP_BUILDER};
use crate::router::Router;
use serde::{Deserialize, Serialize};

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
            next_router.insert(rule.clone().into_route());
        }

        // Add rules
        for rule in &impact.change_set.new {
            next_router.insert(rule.clone().into_route());
        }

        let mut items = Vec::new();

        for example in &impact.examples {
            let mut builder = http::Request::<()>::builder()
                .uri(example.url.as_str())
                .method(match &example.method {
                    None => "GET",
                    Some(method) => method.as_str(),
                })
            ;

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
            let mut request = Request::new(
                STATIC_QUERY_PARAM_SKIP_BUILDER.build_query_param_skipped(match http_request.uri().path_and_query() {
                    None => "",
                    Some(path_and_query) => path_and_query.as_str(),
                }),
                match http_request.uri().host() {
                    None => None,
                    Some(host) => Some(host.to_string()),
                },
                match http_request.uri().scheme_str() {
                    None => None,
                    Some(scheme) => Some(scheme.to_string()),
                },
                example.method.clone(),
            );

            if example.headers.is_some() {
                for header in example.headers.as_ref().unwrap() {
                    request.add_header(header.name.clone(), header.value.clone());
                }
            }

            let trace_before_update = RouterTrace::create_from_router(router, &request, &http_request);
            let trace_after_update = RouterTrace::create_from_router(&next_router, &request, &http_request);

            items.push(ImpactResultItem {
                example: example.clone(),
                trace_before_update,
                trace_after_update,
            });
        }

        items
    }
}
