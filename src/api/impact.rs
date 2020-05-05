use serde::{Deserialize, Serialize};
use crate::api::{Rule, RouterTrace};
use crate::router::Router;

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
            let request_res = http::Request::<()>::builder().uri(url.as_str()).method("GET").body(());

            if request_res.is_err() {
                continue;
            }

            let request = request_res.unwrap();
            let trace_before_update = RouterTrace::create_from_router(router, &request);
            let trace_after_update = RouterTrace::create_from_router(&next_router, &request);

            items.push(ImpactResultItem {
                url: url.clone(),
                trace_before_update,
                trace_after_update,
            });
        }

        items
    }
}
