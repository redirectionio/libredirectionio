use crate::action::Action;
use crate::api::Rule;
use crate::http::Request;
use crate::router::Trace;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TraceAction {
    action: Action,
    rule: Rule,
}

impl TraceAction {
    pub fn from_trace_rules(traces: &[Trace<Rule>], request: &Request) -> Vec<TraceAction> {
        let mut traces_action = Vec::new();
        let mut current_action = Action::default();
        let mut routes = Trace::<Rule>::get_routes_from_traces(traces);

        // Reverse order of sort
        routes.sort_by_key(|&a| a.priority());

        for route in routes {
            let (action_rule_opt, reset, stop, _) = Action::from_route_rule(route, request);

            if let Some(action_rule) = action_rule_opt {
                if reset {
                    current_action = action_rule;
                } else {
                    current_action.merge(action_rule);
                }
            }

            traces_action.push(TraceAction {
                action: current_action.clone(),
                rule: route.handler().clone(),
            });

            if stop {
                return traces_action;
            }
        }

        traces_action
    }
}
