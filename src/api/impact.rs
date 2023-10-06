use crate::action::{Action, UnitTrace};
use crate::api::rules_message::RuleChangeSet;
use crate::api::{Example, Rule};
use crate::http::Header;
use crate::http::Request;
use crate::router::{Router, RouterConfig, Trace};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use url::Url;

const REDIRECTION_CODES: [u16; 4] = [301, 302, 307, 308];

// Input

#[derive(Deserialize, Debug, Clone)]
pub struct ImpactInput {
    pub router_config: RouterConfig,
    pub max_hops: u8,
    pub with_redirection_loop: bool,
    pub rule: Rule,
    pub action: String,
    pub rules: Vec<Rule>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ImpactProjectInput {
    pub max_hops: u8,
    pub with_redirection_loop: bool,
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

#[derive(Serialize, Debug, Clone)]
pub struct RedirectionLoop {
    hops: Vec<RedirectionHop>,
    error: Option<RedirectionError>,
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct RedirectionHop {
    pub url: String,
    pub status_code: u16,
    pub method: String,
}

#[derive(Serialize, Debug, Clone)]
enum RedirectionError {
    AtLeastOneHop,
    TooManyHops,
    Loop,
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
                Some(ImpactOutput::compute_redirection_loop(router, max_hops, &example))
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

    fn compute_redirection_loop(router: &Router<Rule>, max_hops: u8, example: &Example) -> RedirectionLoop {
        let mut current_url = example.url.clone();
        let mut current_method = example.method.clone().unwrap_or(String::from("GET"));
        let mut error = None;

        let mut hops = vec![RedirectionHop {
            url: current_url.clone(),
            status_code: 0,
            method: current_method.clone(),
        }];

        'outer: for i in 1..=max_hops {
            let new_example = example.with_url(current_url.clone()).with_method(Some(current_method.clone()));

            let request = Request::from_example(&router.config, &new_example).unwrap();
            let routes = router.match_request(&request);
            let mut action = Action::from_routes_rule(routes, &request, None);

            let action_status_code = action.get_status_code(0, None);
            let (final_status_code, backend_status_code) = if action_status_code != 0 {
                (action_status_code, action_status_code)
            } else {
                // We call the backend and get a response code
                let backend_status_code = new_example.response_status_code.unwrap_or(200);
                let final_status_code = action.get_status_code(backend_status_code, None);
                (final_status_code, backend_status_code)
            };

            if !REDIRECTION_CODES.contains(&final_status_code) {
                break;
            }

            let headers = action.filter_headers(Vec::new(), backend_status_code, false, None);

            let mut found = false;
            for header in headers.iter() {
                if header.name.to_lowercase() == "location" {
                    current_url = join_url(current_url.as_str(), header.value.as_str());
                    found = true;
                    break;
                }
            }

            if !found {
                break;
            }

            if i > 1 {
                error = Some(RedirectionError::AtLeastOneHop);
            }

            if [301, 302].contains(&final_status_code) {
                current_method = String::from("GET");
            }

            for hop in hops.iter() {
                if hop.url == current_url && hop.method == current_method {
                    hops.push(RedirectionHop {
                        url: current_url,
                        status_code: final_status_code,
                        method: current_method,
                    });
                    error = Some(RedirectionError::Loop);
                    break 'outer;
                }
            }

            hops.push(RedirectionHop {
                url: current_url.clone(),
                status_code: final_status_code,
                method: current_method.clone(),
            });

            if i >= max_hops {
                error = Some(RedirectionError::TooManyHops);
                break;
            }
        }

        RedirectionLoop { hops, error }
    }
}

fn join_url(base: &str, path: &str) -> String {
    let base = match Url::parse(base) {
        Ok(url) => url,
        Err(_) => return path.to_string(),
    };

    let url = match base.join(path) {
        Ok(url) => url,
        Err(_) => return path.to_string(),
    };

    url.to_string()
}
