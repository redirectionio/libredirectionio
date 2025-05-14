use std::sync::Arc;

use serde::{Deserialize, Serialize};

use super::{Example, Rule};
use crate::{
    action::{RunExample, UnitTrace},
    api::{redirection_loop::RedirectionLoop, rules_message::RuleChangeSet},
    http::Header,
    router::{Router, Trace},
    router_config::RouterConfig,
};
// Input

#[derive(Deserialize, Debug, Clone)]
pub struct ExplainRequestInput {
    pub router_config: RouterConfig,
    pub example: Example,
    pub rules: Vec<Rule>,
    pub max_hops: u8,
    #[serde(default)]
    pub project_domains: Vec<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ExplainRequestProjectInput {
    pub example: Example,
    pub change_set: RuleChangeSet,
    pub max_hops: u8,
    #[serde(default)]
    pub project_domains: Vec<String>,
}

// Output

#[derive(Serialize, Debug, Clone)]
pub struct ExplainRequestOutput {
    example: Example,
    unit_trace: UnitTrace,
    backend_status_code: u16,
    response: Response,
    match_traces: Vec<Trace<Rule>>,
    redirection_loop: Option<RedirectionLoop>,
    should_log_request: bool,
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct Response {
    pub status_code: u16,
    pub headers: Vec<Header>,
    pub body: String,
}

#[derive(Serialize, Debug)]
pub struct ExplainRequestOutputError {
    pub message: String,
}

// Implementation

impl ExplainRequestOutput {
    pub fn create_result_from_project(
        explain_request_input: ExplainRequestProjectInput,
        existing_router: Arc<Router<Rule>>,
    ) -> Result<ExplainRequestOutput, ExplainRequestOutputError> {
        let explain_request_router = if explain_request_input.change_set.is_empty() {
            existing_router
        } else {
            Arc::new(explain_request_input.change_set.update_existing_router(existing_router))
        };

        Self::create_result(
            explain_request_router,
            &explain_request_input.example,
            explain_request_input.max_hops,
            explain_request_input.project_domains,
        )
    }

    pub fn create_result_without_project(
        explain_request_input: ExplainRequestInput,
    ) -> Result<ExplainRequestOutput, ExplainRequestOutputError> {
        let mut router = Router::<Rule>::from_config(explain_request_input.router_config);

        for rule in explain_request_input.rules.iter() {
            router.insert(rule.clone());
        }

        Self::create_result(
            Arc::new(router),
            &explain_request_input.example,
            explain_request_input.max_hops,
            explain_request_input.project_domains,
        )
    }

    fn create_result(
        router: Arc<Router<Rule>>,
        example: &Example,
        max_hops: u8,
        project_domains: Vec<String>,
    ) -> Result<ExplainRequestOutput, ExplainRequestOutputError> {
        let run = RunExample::new(router.as_ref(), example).map_err(|e| ExplainRequestOutputError {
            message: format!("invalid example: {e}"),
        })?;

        let redirection_loop = Some(RedirectionLoop::from_example(router.as_ref(), max_hops, example, project_domains));

        Ok(ExplainRequestOutput {
            example: example.to_owned(),
            unit_trace: run.unit_trace,
            backend_status_code: run.backend_status_code,
            response: Response {
                status_code: run.response.status_code,
                headers: run.response.headers,
                body: run.response.body,
            },
            match_traces: router.trace_request(&run.request),
            redirection_loop,
            should_log_request: run.should_log_request,
        })
    }
}
