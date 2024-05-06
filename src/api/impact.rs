use crate::action::{Action, UnitTrace};
use crate::api::redirection_loop::RedirectionLoop;
use crate::api::rules_message::RuleChangeSet;
use crate::api::{Example, Rule};
use crate::http::Header;
use crate::http::Request;
use crate::router::{Router, Trace};
use crate::router_config::RouterConfig;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

// Input

#[derive(Deserialize, Debug, Clone)]
pub struct ImpactInput {
    pub router_config: RouterConfig,
    pub max_hops: u8,
    pub with_redirection_loop: bool,
    #[serde(default)]
    pub domains: Vec<String>,
    pub rule: Rule,
    pub action: String,
    pub rules: Vec<Rule>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ImpactProjectInput {
    pub max_hops: u8,
    pub with_redirection_loop: bool,
    #[serde(default)]
    pub domains: Vec<String>,
    pub rule: Rule,
    pub action: String,
    pub change_set: RuleChangeSet,
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
    redirection_loop: Option<RedirectionLoop>,
    should_log_request: bool,
}

#[derive(Serialize, Debug, Clone, Default)]
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
            redirection_loop: None,
            match_traces: Vec::new(),
            should_log_request: false,
        }
    }
}

impl ImpactOutput {
    pub fn from_impact_project(impact_input: ImpactProjectInput, existing_router: Arc<Router<Rule>>) -> ImpactOutput {
        let mut impact_router = impact_input.change_set.update_existing_router(existing_router.clone());
        let mut trace_unique_router = Router::<Rule>::from_arc_config(existing_router.config.clone());

        impact_router.remove(impact_input.rule.id.as_str());

        ImpactOutput::compute_impacts(
            &mut impact_router,
            &mut trace_unique_router,
            impact_input.rule.examples.clone(),
            impact_input.with_redirection_loop,
            impact_input.max_hops,
            impact_input.action.as_str(),
            impact_input.rule,
            impact_input.domains,
        )
    }

    pub fn create_result(impact_input: ImpactInput) -> ImpactOutput {
        let mut router = Router::<Rule>::from_config(impact_input.router_config.clone());
        let mut trace_unique_router = Router::<Rule>::from_config(impact_input.router_config.clone());

        for rule in impact_input.rules.iter() {
            // Even for a "add" action, we remove a potential previous version
            // of the rule. This occurs when adding a rule (still in draft) and
            // then editing it (still in add / draft). But we want the very last
            // version.
            if rule.id == impact_input.rule.id {
                continue;
            }
            router.insert(rule.clone());
        }

        ImpactOutput::compute_impacts(
            &mut router,
            &mut trace_unique_router,
            impact_input.rule.examples.clone(),
            impact_input.with_redirection_loop,
            impact_input.max_hops,
            impact_input.action.as_str(),
            impact_input.rule,
            impact_input.domains,
        )
    }

    fn compute_impacts(
        router: &mut Router<Rule>,
        trace_unique_router: &mut Router<Rule>,
        examples: Option<Vec<Example>>,
        with_redirection_loop: bool,
        max_hops: u8,
        action: &str,
        rule: Rule,
        project_domains: Vec<String>,
    ) -> ImpactOutput {
        if action == "add" || action == "update" {
            router.insert(rule.clone());
            trace_unique_router.insert(rule);
        }

        let mut impacts = Vec::new();

        if examples.is_none() {
            return ImpactOutput { impacts };
        }

        for example in examples.unwrap() {
            let request = match Request::from_example(&router.config, &example) {
                Ok(request) => request,
                Err(e) => {
                    impacts.push(Impact::new_with_error(
                        example.to_owned(),
                        format!("Cannot create query from example: {e}"),
                    ));

                    continue;
                }
            };

            let mut unit_trace = UnitTrace::default();

            let routes = router.match_request(&request);
            let mut action = Action::from_routes_rule(routes, &request, Some(&mut unit_trace));

            let example_status_code = example.response_status_code.unwrap_or(0);
            let (final_status_code, backend_status_code) =
                action.get_final_status_code_with_fallback(example_status_code, 200, &mut unit_trace);

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

            let redirection_loop = if with_redirection_loop {
                Some(RedirectionLoop::from_example(router, max_hops, &example, project_domains.clone()))
            } else {
                None
            };

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
                redirection_loop,
                should_log_request,
            });
        }

        ImpactOutput { impacts }
    }
}
