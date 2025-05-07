use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::{
    action::{ExampleRun, UnitTrace},
    api::{Example, Rule, redirection_loop::RedirectionLoop, rules_message::RuleChangeSet},
    http::Header,
    router::{Router, Trace},
    router_config::RouterConfig,
};
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

    #[allow(clippy::too_many_arguments)]
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
            let mut run = match ExampleRun::new(router, &example) {
                Ok(r) => r,
                Err(e) => {
                    impacts.push(Impact::new_with_error(
                        example.to_owned(),
                        format!("Cannot create example run: {e}"),
                    ));

                    continue;
                }
            };

            if with_redirection_loop {
                run.with_redirection_loop(router, max_hops, &example, project_domains.clone());
            }

            run.with_match_traces(trace_unique_router);

            impacts.push(Impact {
                example,
                unit_trace: run.unit_trace,
                backend_status_code: run.backend_status_code,
                response: Response {
                    status_code: run.response.status_code,
                    headers: run.response.headers,
                    body: run.response.body,
                },
                match_traces: run.match_traces,
                error: None,
                redirection_loop: run.redirection_loop,
                should_log_request: run.should_log_request,
            });
        }

        ImpactOutput { impacts }
    }
}
