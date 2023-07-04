use crate::action::{Action, UnitTrace};
use crate::api::{Example, Rule};
use crate::http::Header;
use crate::http::Request;
use crate::router::{Router, RouterConfig, Trace};
use serde::{Deserialize, Serialize};
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
    pub fn create_result(impact_input: ImpactInput) -> ImpactOutput {
        let router_config = impact_input.router_config.clone();
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

        let impacts = ImpactOutput::compute_impacts(&router, &trace_unique_router, &impact_input);

        ImpactOutput { impacts }
    }

    fn compute_impacts(router: &Router<Rule>, trace_unique_router: &Router<Rule>, impact_input: &ImpactInput) -> Vec<Impact> {
        let mut impacts = Vec::new();
        for example in impact_input.rule.examples.as_ref().unwrap().iter() {
            let request = match Request::from_example(&router.config, example) {
                Ok(request) => request,
                Err(e) => {
                    impacts.push(Impact::new_with_error(
                        example.to_owned(),
                        format!("Cannot create query from example: {}", e),
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

            let redirection_loop = if impact_input.with_redirection_loop {
                Some(ImpactOutput::compute_redirection_loop(router, impact_input, example))
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
        impacts
    }

    fn compute_redirection_loop(router: &Router<Rule>, impact_input: &ImpactInput, example: &Example) -> RedirectionLoop {
        let mut current_url = example.url.clone();
        let mut current_method = example.method.clone();
        let mut error = None;

        let mut hops = vec![RedirectionHop {
            url: current_url.clone(),
            status_code: 0,
        }];

        'outer: for i in 1..=impact_input.max_hops {
            let new_example = example.with_url(current_url.clone()).with_method(current_method.clone());

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

            for hop in hops.iter() {
                if hop.url == current_url {
                    hops.push(RedirectionHop {
                        url: current_url,
                        status_code: final_status_code,
                    });
                    error = Some(RedirectionError::Loop);
                    break 'outer;
                }
            }

            hops.push(RedirectionHop {
                url: current_url.clone(),
                status_code: final_status_code,
            });

            if i >= impact_input.max_hops {
                error = Some(RedirectionError::TooManyHops);
                break;
            }

            if [301, 302].contains(&final_status_code) {
                current_method = Some(String::from("GET"));
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
