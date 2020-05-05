use serde::{Deserialize, Serialize};
use crate::router::{Router, Trace};
use crate::api::Rule;
use crate::action::TraceAction;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RouterTrace {
    match_traces: Vec<Trace<Rule>>,
    action_traces: Vec<TraceAction>,
}

impl RouterTrace {
    pub fn create_from_router(router: &Router<Rule>, request: &http::Request<()>) -> RouterTrace {
        let match_traces = router.trace_request(&request);
        let action_traces = TraceAction::from_trace_rules(&match_traces, &request);

        RouterTrace {
            match_traces,
            action_traces,
        }
    }
}
