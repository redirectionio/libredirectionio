use std::collections::HashMap;

use crate::action::{Action, UnitTrace};
use crate::api::{Example, Rule};
use crate::http::Request;
use crate::router::{Router, RouterConfig};
use serde::{Deserialize, Serialize};

// Input

#[derive(Deserialize, Debug, Clone)]
pub struct UnitIdsInput {
    pub router_config: RouterConfig,
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
pub struct UnitIdsOutput {
    pub rules: HashMap<String, RuleOutput>,
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct RuleOutput {
    pub examples: Vec<Example>,
}

// Implementation

impl UnitIdsOutput {
    pub fn create_result(unit_ids_input: UnitIdsInput) -> UnitIdsOutput {
        let mut router = Router::<Rule>::from_config(unit_ids_input.router_config.clone());
        let router_config = router.config.clone();

        for rule in unit_ids_input.rules.rules.iter() {
            router.insert(rule.clone().into_route(&router_config));
        }

        router.cache(10000);

        let mut rules = HashMap::new();

        for rule in unit_ids_input.rules.rules {
            if rule.examples.is_none() {
                continue;
            }
            let mut examples = Vec::new();

            for example in rule.examples.unwrap() {
                let request = match Request::from_example(&router_config, &example) {
                    Ok(request) => request,
                    Err(_) => {
                        examples.push(example.clone());
                        continue;
                    }
                };
                let routes = router.match_request(&request);
                let mut action = Action::from_routes_rule(routes, &request);

                let mut unit_trace = UnitTrace::default();

                let action_status_code = action.get_status_code(0, Some(&mut unit_trace));
                let (_, backend_status_code) = if action_status_code != 0 {
                    (action_status_code, action_status_code)
                } else {
                    // We call the backend and get a response code
                    let backend_status_code = example.response_status_code.unwrap_or(200);
                    let final_status_code = action.get_status_code(backend_status_code, Some(&mut unit_trace));
                    (final_status_code, backend_status_code)
                };

                action.filter_headers(Vec::new(), backend_status_code, false, Some(&mut unit_trace));

                let body = "<!DOCTYPE html>
<html>
    <head>
    </head>
    <body>
    </body>
</html>";
                if let Some(mut body_filter) = action.create_filter_body(backend_status_code, &[]) {
                    body_filter.filter(body.into(), Some(&mut unit_trace));
                    body_filter.end(Some(&mut unit_trace));
                }

                unit_trace.squash_with_target_unit_traces();

                let mut final_example = example.clone();
                final_example.unit_ids_applied = Some(unit_trace.get_unit_ids_applied().into_iter().collect());
                examples.push(final_example);
            }

            rules.insert(rule.id.clone(), RuleOutput { examples });
        }
        UnitIdsOutput { rules }
    }
}