#[cfg(not(target_arch = "wasm32"))]
mod ffi;
mod status_code_update;
pub mod wasm;

use crate::api::{BodyFilter, HeaderFilter, Rule};
use crate::filter::{FilterBodyAction, FilterHeaderAction};
use crate::http::{Header, Request};
use crate::router::{Route, StaticOrDynamic, Trace};
use serde::{Deserialize, Serialize};
pub use status_code_update::StatusCodeUpdate;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Action {
    pub status_code_update: Option<StatusCodeUpdate>,
    header_filters: Vec<HeaderFilterAction>,
    body_filters: Vec<BodyFilterAction>,
    pub rule_ids: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TraceAction {
    action: Action,
    rule: Rule,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct HeaderFilterAction {
    filter: HeaderFilter,
    on_response_status_code: u16,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct BodyFilterAction {
    filter: BodyFilter,
    on_response_status_code: u16,
}

impl Default for Action {
    fn default() -> Action {
        Action {
            status_code_update: None,
            header_filters: Vec::new(),
            body_filters: Vec::new(),
            rule_ids: Vec::new(),
        }
    }
}

impl TraceAction {
    pub fn from_trace_rules(traces: &[Trace<Rule>], request: &Request) -> Vec<TraceAction> {
        let mut traces_action = Vec::new();
        let mut current_action = Action::default();
        let mut routes = Trace::<Rule>::get_routes_from_traces(traces);

        // Reverse order of sort
        routes.sort_by(|a, b| a.priority().cmp(&b.priority()));

        for route in routes {
            current_action.merge(Action::from_route_rule(route, request));

            traces_action.push(TraceAction {
                action: current_action.clone(),
                rule: route.handler().clone(),
            })
        }

        traces_action
    }
}

impl Action {
    fn get_parameters(route: &Route<Rule>, request: &Request) -> HashMap<String, String> {
        let path = request.path_and_query.path_and_query.clone();
        let mut parameters = route.path_and_query().capture(path.as_str());

        if let Some(host) = route.host() {
            if let Some(request_host) = request.host.as_ref() {
                parameters.extend(host.capture(request_host));
            }
        }

        for header in route.headers() {
            for request_header in &request.headers {
                if request_header.name != header.name {
                    continue;
                }

                parameters.extend(header.capture(request_header.value.as_str()));
            }
        }

        parameters
    }

    pub fn from_route_rule(route: &Route<Rule>, request: &Request) -> Action {
        let parameters = Self::get_parameters(route, request);
        let rule = route.handler();
        let transformers = rule.transformers();
        let status_code_update = match rule.redirect_code.unwrap_or(0) {
            0 => None,
            redirect_code => Some(StatusCodeUpdate {
                status_code: redirect_code,
                on_response_status_code: rule.match_on_response_status.unwrap_or(0),
                fallback_status_code: 0,
            }),
        };

        let mut header_filters = Vec::new();
        let mut body_filters = Vec::new();

        if let Some(target) = &rule.target {
            if !target.is_empty() {
                let mut value = StaticOrDynamic::replace(target.clone(), &parameters, &transformers);

                if let Some(skipped_query_params) = request.path_and_query.skipped_query_params.as_ref() {
                    if value.contains('?') {
                        value.push_str("&");
                        value.push_str(skipped_query_params.as_str());
                    } else {
                        value.push_str("?");
                        value.push_str(skipped_query_params.as_str());
                    }
                }

                header_filters.push(HeaderFilterAction {
                    filter: HeaderFilter {
                        action: "override".to_string(),
                        value,
                        header: "Location".to_string(),
                    },
                    on_response_status_code: match rule.match_on_response_status {
                        None => 0,
                        Some(on_response) => match rule.redirect_code {
                            None => on_response,
                            Some(redirect_code) => redirect_code,
                        },
                    },
                })
            }
        }

        if let Some(rule_header_filters) = rule.header_filters.as_ref() {
            for filter in rule_header_filters {
                header_filters.push(HeaderFilterAction {
                    filter: HeaderFilter {
                        action: filter.action.clone(),
                        header: filter.header.clone(),
                        value: StaticOrDynamic::replace(filter.value.clone(), &parameters, &transformers),
                    },
                    on_response_status_code: rule.match_on_response_status.unwrap_or(0),
                });
            }
        }

        if let Some(rule_body_filters) = rule.body_filters.as_ref() {
            for filter in rule_body_filters {
                body_filters.push(BodyFilterAction {
                    filter: BodyFilter {
                        action: filter.action.clone(),
                        css_selector: filter.css_selector.clone(),
                        element_tree: filter.element_tree.clone(),
                        value: StaticOrDynamic::replace(filter.value.clone(), &parameters, &transformers),
                    },
                    on_response_status_code: rule.match_on_response_status.unwrap_or(0),
                });
            }
        }

        Action {
            status_code_update,
            header_filters,
            body_filters,
            rule_ids: vec![rule.id.clone()],
        }
    }

    pub fn merge(&mut self, other: Self) {
        self.status_code_update = match other.status_code_update {
            None => self.status_code_update.clone(),
            Some(new_status_code_update) => match &self.status_code_update {
                None => Some(new_status_code_update),
                Some(old_status_code_update) => {
                    if old_status_code_update.on_response_status_code != 0 || new_status_code_update.on_response_status_code == 0 {
                        Some(new_status_code_update)
                    } else {
                        Some(StatusCodeUpdate {
                            status_code: new_status_code_update.status_code,
                            on_response_status_code: new_status_code_update.on_response_status_code,
                            fallback_status_code: old_status_code_update.status_code,
                        })
                    }
                }
            },
        };

        for filter in other.header_filters {
            self.header_filters.push(filter);
        }

        for filter in other.body_filters {
            self.body_filters.push(filter);
        }

        for rule_id in other.rule_ids {
            self.rule_ids.push(rule_id)
        }
    }

    pub fn from_routes_rule(mut routes: Vec<&Route<Rule>>, request: &Request) -> Action {
        let mut action = Action::default();

        // Sort by priority, with lower ones in first
        routes.sort_by(|a, b| a.priority().cmp(&b.priority()));

        for route in routes {
            action.merge(Action::from_route_rule(route, request));
        }

        action
    }

    pub fn get_status_code(&self, response_status_code: u16) -> u16 {
        match self.status_code_update.as_ref() {
            None => 0,
            Some(status_code_update) => status_code_update.get_status_code(response_status_code),
        }
    }

    pub fn filter_headers(&self, headers: Vec<Header>, response_status_code: u16, add_rule_ids_header: bool) -> Vec<Header> {
        let mut filters = Vec::new();

        for filter in &self.header_filters {
            if filter.on_response_status_code != 0 && filter.on_response_status_code != response_status_code {
                continue;
            }

            filters.push(&filter.filter);
        }

        let mut new_headers = match FilterHeaderAction::new(filters) {
            None => headers,
            Some(filter_action) => filter_action.filter(headers),
        };

        if add_rule_ids_header {
            new_headers.push(Header {
                name: "X-RedirectionIo-RuleIds".to_string(),
                value: self.rule_ids.join(";"),
            });
        }

        new_headers
    }

    pub fn create_filter_body(&self, response_status_code: u16) -> Option<FilterBodyAction> {
        let mut filters = Vec::new();

        for filter in &self.body_filters {
            if filter.on_response_status_code != 0 && filter.on_response_status_code != response_status_code {
                continue;
            }

            filters.push(&filter.filter);
        }

        FilterBodyAction::new(filters)
    }
}
