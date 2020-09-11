use crate::api::{RouterTrace, Rule};
use crate::router::Router;
use serde::{Deserialize, Serialize};
use crate::http::{Request, STATIC_QUERY_PARAM_SKIP_BUILDER};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Impact {
    urls: Vec<String>,
    change_set: ChangeSet,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImpactResultItem {
    url: String,
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
            next_router.insert(rule.clone().into_route());
        }

        // Add rules
        for rule in &impact.change_set.new {
            next_router.insert(rule.clone().into_route());
        }

        let mut items = Vec::new();

        for url in &impact.urls {
            let http_request_res = http::Request::<()>::builder().uri(url.as_str()).method("GET").body(());

            if http_request_res.is_err() {
                continue;
            }

            let http_request = http_request_res.unwrap();
            let request = Request::new(
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
                None,
            );

            let trace_before_update = RouterTrace::create_from_router(router, &request, &http_request);
            let trace_after_update = RouterTrace::create_from_router(&next_router, &request, &http_request);

            items.push(ImpactResultItem {
                url: url.clone(),
                trace_before_update,
                trace_after_update,
            });
        }

        items
    }
}
