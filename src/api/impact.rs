use crate::action::{Action, UnitTrace};
use crate::api::{Example, Rule};
use crate::http::Header;
use crate::http::Request;
use crate::router::{Router, RouterConfig, Trace};
use serde::{Deserialize, Serialize};

// Input

#[derive(Deserialize, Debug, Clone)]
pub struct ImpactInput {
    pub router_config: RouterConfig,
    pub rule: Rule,
    pub action: String,
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

#[derive(Serialize, Debug, Clone, Default)]
pub struct ImpactOutput {
    pub impacts: Vec<Impact>,
}

#[derive(Serialize, Debug, Clone)]
pub struct Impact {
    example: Example,
    unit_trace: UnitTrace,
    backend_status_code: u16,
    response: Response,
    match_traces: Vec<Trace<Rule>>,
    error: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Response {
    pub status_code: u16,
    pub headers: Vec<Header>,
    pub body: String,
}

// Implementation

impl Impact {
    fn new_with_error(example: Example, error: String) -> Self {
        Impact {
            example,
            error: Some(error),
            unit_trace: UnitTrace::default(),
            backend_status_code: 0,
            response: Response::default(),
            match_traces: Vec::new(),
        }
    }
}

impl ImpactOutput {
    pub fn create_result(impact_input: ImpactInput) -> ImpactOutput {
        let router_config = impact_input.router_config;
        let mut router = Router::<Rule>::from_config(router_config.clone());
        let mut trace_unique_router = Router::<Rule>::from_config(router_config.clone());

        for rule in impact_input.rules.rules.iter() {
            // Even for a "add" action, we remove a potential previous version
            // of the rule. This occurs when adding a rule (still in draft) and
            // then editing it (still in add / draft). But we want the very last
            // version.
            if rule.id == impact_input.rule.id {
                continue;
            }
            router.insert(rule.clone().into_route(&router_config));
        }

        if impact_input.action == "add" || impact_input.action == "update" {
            router.insert(impact_input.rule.clone().into_route(&router_config));
            trace_unique_router.insert(impact_input.rule.clone().into_route(&router_config));
        }

        let mut impacts = Vec::new();

        for example in impact_input.rule.examples.as_ref().unwrap().iter() {
            let request = match Request::from_example(&router_config, example) {
                Ok(request) => request,
                Err(e) => {
                    impacts.push(Impact::new_with_error(
                        example.to_owned(),
                        format!("Cannot create query from example: {}", e),
                    ));

                    continue;
                }
            };
            let routes = router.match_request(&request);
            let mut action = Action::from_routes_rule(routes, &request);

            let mut unit_trace = UnitTrace::default();

            let action_status_code = action.get_status_code(0, Some(&mut unit_trace));
            let (final_status_code, backend_status_code) = if action_status_code != 0 {
                (action_status_code, action_status_code)
            } else {
                // We call the backend and get a response code
                let backend_status_code = example.response_status_code.unwrap_or(200);
                let final_status_code = action.get_status_code(backend_status_code, Some(&mut unit_trace));
                (final_status_code, backend_status_code)
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

            unit_trace.squash_with_target_unit_traces();

            impacts.push(Impact {
                example: example.to_owned(),
                unit_trace,
                backend_status_code,
                response: Response {
                    status_code: final_status_code,
                    headers,
                    body: body.to_string(),
                },
                match_traces: trace_unique_router.trace_request(&request),
                error: None,
            });
        }

        ImpactOutput { impacts }
    }
}
