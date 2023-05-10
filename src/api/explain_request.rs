use crate::{
    action::{Action, UnitTrace},
    http::{Header, Request},
    router::{Router, RouterConfig, Trace},
};

use super::{Example, Rule};
use serde::{Deserialize, Serialize};

// Input

#[derive(Deserialize, Debug, Clone)]
pub struct ExplainRequestInput {
    pub router_config: RouterConfig,
    pub example: Example,
    pub rules: TmpRules,
}
// FIXME: find a way to avoid creating this structure.
// It would be more convenient to inline the structure
#[derive(Deserialize, Debug, Clone)]
pub struct TmpRules {
    #[serde(rename = "hydra:member")]
    pub rules: Vec<Rule>,
}

// Output

#[derive(Serialize, Debug, Clone)]
pub struct ExplainRequestOutput {
    example: Example,
    unit_trace: UnitTrace,
    backend_status_code: u16,
    response: Response,
    match_traces: Vec<Trace<Rule>>,
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
    pub fn create_result(explain_request_input: ExplainRequestInput) -> Result<ExplainRequestOutput, ExplainRequestOutputError> {
        let router_config = explain_request_input.router_config;
        let mut router = Router::<Rule>::from_config(router_config.clone());

        for rule in explain_request_input.rules.rules.iter() {
            router.insert(rule.clone().into_route(&router_config));
        }

        let example = &explain_request_input.example;

        let request = match Request::from_example(&router_config, example) {
            Ok(request) => request,
            Err(e) => {
                return Err(ExplainRequestOutputError {
                    message: format!("Invalid example: {}", e),
                });
            }
        };
        let mut unit_trace = UnitTrace::default();

        let routes = router.match_request(&request);
        let mut action = Action::from_routes_rule(routes, &request, Some(&mut unit_trace));

        let example_status_code = example.response_status_code.unwrap_or(0);
        let action_status_code = action.get_status_code(example_status_code, Some(&mut unit_trace));
        let (final_status_code, backend_status_code) = if action_status_code != 0 {
            (action_status_code, action_status_code)
        } else {
            let backend_status_code = example.response_status_code.unwrap_or(200);
            let final_status_code = action.get_status_code(backend_status_code, Some(&mut unit_trace));
            (final_status_code, example_status_code)
        };

        let headers = action.filter_headers(Vec::new(), backend_status_code, false, Some(&mut unit_trace));

        let mut body = "<!DOCTYPE html>
<html>
    <head>
    </head>
    <body>
    </body>
</html>";

        let mut b1;
        if let Some(mut body_filter) = action.create_filter_body(backend_status_code, &[]) {
            b1 = body_filter.filter(body.into(), Some(&mut unit_trace));
            let b2 = body_filter.end(Some(&mut unit_trace));
            b1.extend(b2);
            body = std::str::from_utf8(&b1).unwrap();
        }

        let should_log_request = action.should_log_request(true, final_status_code, Some(&mut unit_trace));

        unit_trace.squash_with_target_unit_traces();

        Ok(ExplainRequestOutput {
            example: example.to_owned(),
            unit_trace,
            backend_status_code,
            response: Response {
                status_code: final_status_code,
                headers,
                body: body.to_string(),
            },
            match_traces: router.trace_request(&request),
            should_log_request,
        })
    }
}
