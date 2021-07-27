#[cfg(not(target_arch = "wasm32"))]
mod ffi;
mod log_override;
mod status_code_update;
pub mod wasm;

use crate::action::log_override::LogOverride;
use crate::api::{BodyFilter, HeaderFilter, Rule};
use crate::filter::{FilterBodyAction, FilterHeaderAction};
use crate::http::{Header, Request};
use crate::router::{Route, StaticOrDynamic, Trace};
use serde::{Deserialize, Serialize};
pub use status_code_update::StatusCodeUpdate;
use std::collections::HashSet;
use std::iter::FromIterator;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Action {
    status_code_update: Option<StatusCodeUpdate>,
    header_filters: Vec<HeaderFilterAction>,
    body_filters: Vec<BodyFilterAction>,
    // In 3.0 remove this
    rule_ids: Vec<String>,
    // In 3.0 make this mandatory
    rule_traces: Option<Vec<RuleTrace>>,
    // In 3.0 make this mandatory
    rules_applied: Option<HashSet<String>>,
    // In 3.0 make this mandatory
    log_override: Option<LogOverride>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RuleTrace {
    id: String,
    on_response_status_codes: Vec<u16>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TraceAction {
    action: Action,
    rule: Rule,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct HeaderFilterAction {
    filter: HeaderFilter,
    on_response_status_codes: Vec<u16>,
    rule_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct BodyFilterAction {
    filter: BodyFilter,
    on_response_status_codes: Vec<u16>,
    rule_id: Option<String>,
}

impl Default for Action {
    fn default() -> Action {
        Action {
            status_code_update: None,
            header_filters: Vec::new(),
            body_filters: Vec::new(),
            rule_ids: Vec::new(),
            rule_traces: Some(Vec::new()),
            rules_applied: Some(HashSet::new()),
            log_override: None,
        }
    }
}

impl TraceAction {
    pub fn from_trace_rules(traces: &[Trace<Rule>], request: &Request) -> Vec<TraceAction> {
        let mut traces_action = Vec::new();
        let mut current_action = Action::default();
        let mut routes = Trace::<Rule>::get_routes_from_traces(traces);

        // Reverse order of sort
        routes.sort_by_key(|&a| a.priority());

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
    pub fn get_applied_rule_ids(&self) -> Vec<String> {
        match &self.rules_applied {
            None => self.rule_ids.clone(),
            Some(rules) => Vec::from_iter(rules.clone()),
        }
    }

    pub fn from_route_rule(route: &Route<Rule>, request: &Request) -> Action {
        let markers_captured = route.capture(request);
        let variables = route.handler().variables(&markers_captured, request);
        let rule = route.handler();
        let on_response_status_codes = match rule.source.response_status_codes.as_ref() {
            None => Vec::new(),
            Some(codes) => codes.clone(),
        };

        let status_code_update = match rule.redirect_code.unwrap_or(0) {
            0 => None,
            redirect_code => Some(StatusCodeUpdate {
                status_code: redirect_code,
                on_response_status_codes: on_response_status_codes.clone(),
                fallback_status_code: 0,
                rule_id: Some(rule.id.clone()),
                fallback_rule_id: None,
            }),
        };

        let mut header_filters = Vec::new();
        let mut body_filters = Vec::new();

        if let Some(target) = &rule.target {
            if !target.is_empty() {
                let mut value = StaticOrDynamic::replace(target.clone(), &variables);

                if let Some(skipped_query_params) = request.path_and_query_skipped.skipped_query_params.as_ref() {
                    if value.contains('?') {
                        value.push('&');
                    } else {
                        value.push('?');
                    }

                    value.push_str(skipped_query_params.as_str());
                }

                header_filters.push(HeaderFilterAction {
                    filter: HeaderFilter {
                        action: "override".to_string(),
                        value,
                        header: "Location".to_string(),
                    },
                    on_response_status_codes: match rule.source.response_status_codes.as_ref() {
                        None => Vec::new(),
                        Some(on_response) => match rule.redirect_code {
                            None => on_response.clone(),
                            Some(redirect_code) => vec![redirect_code],
                        },
                    },
                    rule_id: Some(rule.id.clone()),
                })
            }
        }

        if let Some(rule_header_filters) = rule.header_filters.as_ref() {
            for filter in rule_header_filters {
                header_filters.push(HeaderFilterAction {
                    filter: HeaderFilter {
                        action: filter.action.clone(),
                        header: filter.header.clone(),
                        value: StaticOrDynamic::replace(filter.value.clone(), &variables),
                    },
                    on_response_status_codes: on_response_status_codes.clone(),
                    rule_id: Some(rule.id.clone()),
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
                        value: StaticOrDynamic::replace(filter.value.clone(), &variables),
                    },
                    on_response_status_codes: on_response_status_codes.clone(),
                    rule_id: Some(rule.id.clone()),
                });
            }
        }

        Action {
            status_code_update,
            header_filters,
            body_filters,
            rule_ids: vec![rule.id.clone()],
            rule_traces: Some(vec![RuleTrace {
                on_response_status_codes: on_response_status_codes.clone(),
                id: rule.id.clone(),
            }]),
            rules_applied: Some(HashSet::new()),
            log_override: rule.log_override.map(|log_override| LogOverride {
                log_override,
                rule_id: Some(rule.id.clone()),
                on_response_status_codes: on_response_status_codes.clone(),
                fallback_log_override: None,
                fallback_rule_id: None,
            }),
        }
    }

    pub fn merge(&mut self, other: Self) {
        self.status_code_update = match other.status_code_update {
            None => self.status_code_update.clone(),
            Some(new_status_code_update) => match &self.status_code_update {
                None => Some(new_status_code_update),
                Some(old_status_code_update) => {
                    if !old_status_code_update.on_response_status_codes.is_empty()
                        || new_status_code_update.on_response_status_codes.is_empty()
                    {
                        Some(new_status_code_update)
                    } else {
                        Some(StatusCodeUpdate {
                            status_code: new_status_code_update.status_code,
                            on_response_status_codes: new_status_code_update.on_response_status_codes,
                            fallback_status_code: old_status_code_update.status_code,
                            rule_id: new_status_code_update.rule_id,
                            fallback_rule_id: old_status_code_update.rule_id.clone(),
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

        if let Some(other_rule_traces) = other.rule_traces {
            for rule_trace in other_rule_traces {
                let self_rule_traces = self.rule_traces.get_or_insert(Vec::new());
                self_rule_traces.push(rule_trace);
            }
        }

        if let Some(other_log_override) = other.log_override {
            self.log_override = match &self.log_override {
                None => Some(other_log_override),
                Some(self_log_override) => {
                    if !self_log_override.on_response_status_codes.is_empty() || other_log_override.on_response_status_codes.is_empty() {
                        Some(other_log_override)
                    } else {
                        Some(LogOverride {
                            log_override: other_log_override.log_override,
                            rule_id: other_log_override.rule_id,
                            on_response_status_codes: other_log_override.on_response_status_codes,
                            fallback_log_override: Some(self_log_override.log_override),
                            fallback_rule_id: self_log_override.rule_id.clone(),
                        })
                    }
                }
            }
        }
    }

    pub fn from_routes_rule(mut routes: Vec<&Route<Rule>>, request: &Request) -> Action {
        let mut action = Action::default();

        // Sort by priority, with lower ones in first
        routes.sort_by_key(|&a| a.priority());

        for route in routes {
            action.merge(Action::from_route_rule(route, request));
        }

        action
    }

    pub fn get_status_code(&mut self, response_status_code: u16) -> u16 {
        match self.status_code_update.as_ref() {
            None => 0,
            Some(status_code_update) => {
                let (status, rule_applied) = status_code_update.get_status_code(response_status_code);

                self.apply_rule_id(rule_applied);

                status
            }
        }
    }

    pub fn filter_headers(&mut self, headers: Vec<Header>, response_status_code: u16, add_rule_ids_header: bool) -> Vec<Header> {
        let mut filters = Vec::new();
        let mut rule_applied = false;

        if let Some(self_rule_traces) = self.rule_traces.clone() {
            rule_applied = true;

            for trace in self_rule_traces {
                if trace.on_response_status_codes.iter().all(|v| *v == response_status_code) {
                    self.apply_rule_id(Some(trace.id));
                }
            }
        }

        for filter in self.header_filters.clone() {
            if !filter.on_response_status_codes.is_empty() && filter.on_response_status_codes.iter().all(|v| *v != response_status_code) {
                continue;
            }

            filters.push(filter.filter);

            if !rule_applied {
                self.apply_rule_id(filter.rule_id);
            }
        }

        let mut new_headers = match FilterHeaderAction::new(filters) {
            None => headers,
            Some(filter_action) => filter_action.filter(headers),
        };

        if add_rule_ids_header {
            new_headers.push(Header {
                name: "X-RedirectionIo-RuleIds".to_string(),
                value: self.get_applied_rule_ids().join(";"),
            });
        }

        new_headers
    }

    pub fn create_filter_body(&mut self, response_status_code: u16) -> Option<FilterBodyAction> {
        let mut filters = Vec::new();
        let rule_applied = self.rule_traces.is_some();

        for filter in self.body_filters.clone() {
            if !filter.on_response_status_codes.is_empty() && filter.on_response_status_codes.iter().all(|v| *v != response_status_code) {
                continue;
            }

            if !rule_applied {
                self.apply_rule_id(filter.rule_id);
            }

            filters.push(filter.filter);
        }

        FilterBodyAction::new(filters)
    }

    pub fn should_log_request(&mut self, allow_log_config: bool, response_status_code: u16) -> bool {
        let rule_applied = self.rule_traces.is_some();

        match self.log_override.as_ref() {
            None => allow_log_config,
            Some(log_override) => {
                let (allow_log, rule_applied_id) = log_override.get_log_override(response_status_code);

                if !rule_applied {
                    self.apply_rule_id(rule_applied_id);
                }

                allow_log.unwrap_or(allow_log_config)
            }
        }
    }

    fn apply_rule_id(&mut self, rule_id: Option<String>) {
        if rule_id.is_none() {
            return;
        }

        self.rules_applied.get_or_insert(HashSet::new()).insert(rule_id.unwrap());
    }
}
