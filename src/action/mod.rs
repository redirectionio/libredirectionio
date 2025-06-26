#[cfg(not(target_arch = "wasm32"))]
mod ffi;
mod log_override;
mod run;
mod status_code_update;
#[cfg(feature = "router")]
mod trace;
mod unit_trace;

#[cfg(feature = "router")]
use std::sync::Arc;
use std::{cell::RefCell, fmt::Debug, iter::FromIterator, rc::Rc};

use linked_hash_set::LinkedHashSet;
pub use run::RunExample;
use serde::{Deserialize, Serialize};
pub use status_code_update::StatusCodeUpdate;
#[cfg(feature = "router")]
pub use trace::TraceAction;

pub use crate::action::unit_trace::UnitTrace;
#[cfg(feature = "router")]
use crate::api::Rule;
#[cfg(feature = "router")]
use crate::api::{HTMLBodyFilter, TextBodyFilter};
#[cfg(feature = "router")]
use crate::http::Request;
#[cfg(feature = "router")]
use crate::marker::StaticOrDynamic;
#[cfg(feature = "router")]
use crate::router::Route;
use crate::{
    action::log_override::LogOverride,
    api::{BodyFilter, HeaderFilter},
    filter::{FilterBodyAction, FilterHeaderAction},
    http::Header,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Action {
    status_code_update: Option<StatusCodeUpdate>,
    header_filters: Vec<HeaderFilterAction>,
    body_filters: Vec<BodyFilterAction>,
    // In 3.0 remove this
    pub rule_ids: LinkedHashSet<String>,
    #[serde(default)]
    rule_traces: Vec<RuleTrace>,
    #[serde(default)]
    pub rules_applied: LinkedHashSet<String>,
    log_override: Option<LogOverride>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RuleTrace {
    id: String,
    on_response_status_codes: Vec<u16>,
    exclude_response_status_codes: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct HeaderFilterAction {
    filter: HeaderFilter,
    on_response_status_codes: Vec<u16>,
    exclude_response_status_codes: bool,
    rule_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct BodyFilterAction {
    filter: BodyFilter,
    on_response_status_codes: Vec<u16>,
    exclude_response_status_codes: bool,
    rule_id: Option<String>,
}

impl Default for Action {
    fn default() -> Action {
        Action {
            status_code_update: None,
            header_filters: Vec::new(),
            body_filters: Vec::new(),
            rule_ids: LinkedHashSet::new(),
            rule_traces: Vec::new(),
            rules_applied: LinkedHashSet::new(),
            log_override: None,
        }
    }
}

impl Action {
    pub fn get_applied_rule_ids(&self) -> &LinkedHashSet<String> {
        &self.rules_applied
    }

    #[cfg(feature = "router")]
    pub fn get_target(route: &Route<Rule>, request: &Request) -> Option<String> {
        let markers_captured = route.capture(request);
        let variables = route.handler().variables(&markers_captured, request);
        let rule = route.handler();

        rule.target.as_ref().map(|t| {
            let mut value = StaticOrDynamic::replace(t.clone(), &variables);

            if let Some(skipped_query_params) = request.path_and_query_skipped.skipped_query_params.as_ref() {
                if value.contains('?') {
                    value.push('&');
                } else {
                    value.push('?');
                }

                value.push_str(skipped_query_params.as_str());
            }

            value
        })
    }

    #[cfg(feature = "router")]
    pub fn from_route_rule(route: Arc<Route<Rule>>, request: &Request) -> (Option<Action>, bool, bool, Option<String>) {
        let markers_captured = route.capture(request);
        let variables = route.handler().variables(&markers_captured, request);
        let rule = route.handler();

        if let Some(sampling) = rule.source.sampling {
            let percent_rand = sampling.clamp(0, 100);
            let random_value = (rand::random::<u32>() % 100) + 1;

            match (request.sampling_override, random_value > percent_rand) {
                (Some(false), _) => return (None, false, false, None),
                (None, true) => return (None, false, false, None),
                _ => (),
            }
        }

        let on_response_status_codes = match rule.source.response_status_codes.as_ref() {
            None => Vec::new(),
            Some(codes) => codes.clone(),
        };

        let status_code_update = match rule.status_code.unwrap_or(0) {
            0 => None,
            redirect_code => Some(StatusCodeUpdate {
                status_code: redirect_code,
                on_response_status_codes: on_response_status_codes.clone(),
                exclude_response_status_codes: rule.source.exclude_response_status_codes.is_some(),
                fallback_status_code: 0,
                rule_id: Some(rule.id.clone()),
                fallback_rule_id: None,
                unit_id: rule.redirect_unit_id.clone(),
                target_hash: Some("status_code".to_string()),
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
                        id: rule.redirect_unit_id.clone(),
                        target_hash: rule.target_hash.clone(),
                    },
                    on_response_status_codes: match rule.source.response_status_codes.as_ref() {
                        None => Vec::new(),
                        Some(on_response) => on_response.clone(),
                    },
                    exclude_response_status_codes: rule.source.exclude_response_status_codes.is_some(),
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
                        id: filter.id.clone(),
                        target_hash: filter.target_hash.clone(),
                    },
                    on_response_status_codes: on_response_status_codes.clone(),
                    exclude_response_status_codes: rule.source.exclude_response_status_codes.is_some(),
                    rule_id: Some(rule.id.clone()),
                });
            }
        }

        if let Some(rule_body_filters) = rule.body_filters.as_ref() {
            for filter in rule_body_filters {
                body_filters.push(BodyFilterAction {
                    filter: match filter {
                        BodyFilter::HTML(html_body_filter) => BodyFilter::HTML(HTMLBodyFilter {
                            action: html_body_filter.action.clone(),
                            css_selector: html_body_filter.css_selector.clone(),
                            element_tree: html_body_filter.element_tree.clone(),
                            value: StaticOrDynamic::replace(html_body_filter.value.clone(), &variables),
                            inner_value: Some(StaticOrDynamic::replace(
                                html_body_filter
                                    .inner_value
                                    .clone()
                                    .unwrap_or_else(|| html_body_filter.value.clone()),
                                &variables,
                            )),
                            id: html_body_filter.id.clone(),
                            target_hash: html_body_filter.target_hash.clone(),
                        }),
                        BodyFilter::Text(text_body_filter) => BodyFilter::Text(TextBodyFilter {
                            action: text_body_filter.action.clone(),
                            content: StaticOrDynamic::replace(text_body_filter.content.clone(), &variables),
                            id: text_body_filter.id.clone(),
                            target_hash: text_body_filter.target_hash.clone(),
                        }),
                    },
                    on_response_status_codes: on_response_status_codes.clone(),
                    exclude_response_status_codes: rule.source.exclude_response_status_codes.is_some(),
                    rule_id: Some(rule.id.clone()),
                });
            }
        }

        let action = Action {
            status_code_update,
            header_filters,
            body_filters,
            rule_ids: LinkedHashSet::from_iter(vec![rule.id.clone()]),
            rule_traces: vec![RuleTrace {
                on_response_status_codes: on_response_status_codes.clone(),
                exclude_response_status_codes: rule.source.exclude_response_status_codes.is_some(),
                id: rule.id.clone(),
            }],
            rules_applied: LinkedHashSet::new(),
            log_override: rule.log_override.map(|log_override| LogOverride {
                log_override,
                rule_id: Some(rule.id.clone()),
                on_response_status_codes: on_response_status_codes.clone(),
                exclude_response_status_codes: rule.source.exclude_response_status_codes.is_some(),
                fallback_log_override: None,
                fallback_rule_id: None,
                unit_id: rule.configuration_log_unit_id.clone(),
            }),
        };

        (
            Some(action),
            rule.reset.unwrap_or(false),
            rule.stop.unwrap_or(false),
            rule.configuration_reset_unit_id.clone(),
        )
    }

    #[cfg(feature = "router")]
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
                            exclude_response_status_codes: new_status_code_update.exclude_response_status_codes,
                            fallback_status_code: old_status_code_update.status_code,
                            rule_id: new_status_code_update.rule_id,
                            target_hash: new_status_code_update.target_hash,
                            fallback_rule_id: old_status_code_update.rule_id.clone(),
                            unit_id: new_status_code_update.unit_id,
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
            self.rule_ids.insert(rule_id);
        }

        for rule_trace in other.rule_traces {
            self.rule_traces.push(rule_trace);
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
                            exclude_response_status_codes: other_log_override.exclude_response_status_codes,
                            fallback_log_override: Some(self_log_override.log_override),
                            fallback_rule_id: self_log_override.rule_id.clone(),
                            unit_id: self_log_override.unit_id.clone(),
                        })
                    }
                }
            }
        }
    }

    #[cfg(feature = "router")]
    pub fn from_routes_rule(mut routes: Vec<Arc<Route<Rule>>>, request: &Request, unit_trace: Option<Rc<RefCell<UnitTrace>>>) -> Action {
        let mut action = Action::default();
        routes.sort();

        for route in routes {
            let (action_rule_opt, reset, stop, configuration_unit_id) = Action::from_route_rule(route, request);

            if let Some(action_rule) = action_rule_opt {
                if reset {
                    if let (Some(trace), Some(unit_id)) = (&unit_trace, &configuration_unit_id) {
                        trace.borrow_mut().add_unit_id_with_target("configuration::reset", unit_id.as_str());
                    }
                    action = action_rule;
                } else {
                    action.merge(action_rule);
                }

                if stop {
                    if let (Some(trace), Some(unit_id)) = (&unit_trace, &configuration_unit_id) {
                        trace.borrow_mut().add_unit_id_with_target("configuration::stop", unit_id.as_str());
                    }
                    return action;
                }
            }
        }

        action
    }

    pub fn get_final_status_code_with_fallback(
        &mut self,
        response_status_code: u16,
        fallback_status_code: u16,
        unit_trace: Rc<RefCell<UnitTrace>>,
    ) -> (u16, u16) {
        let action_status_code = self.get_status_code(response_status_code, Some(unit_trace.clone()));
        if response_status_code == 0 && action_status_code == 0 {
            let final_status_code = self.get_status_code(fallback_status_code, Some(unit_trace));
            (final_status_code, fallback_status_code)
        } else {
            (action_status_code, response_status_code)
        }
    }

    pub fn get_status_code(&mut self, response_status_code: u16, unit_trace: Option<Rc<RefCell<UnitTrace>>>) -> u16 {
        match self.status_code_update.as_ref() {
            None => 0,
            Some(status_code_update) => {
                let (status, rule_applied) = status_code_update.get_status_code(response_status_code);

                if let Some(rule_id) = rule_applied {
                    if let Some(trace) = unit_trace {
                        trace.borrow_mut().rule_ids_applied.insert(rule_id.to_string());

                        if let (Some(target_hash), Some(unit_id)) = (&status_code_update.target_hash, &status_code_update.unit_id) {
                            trace.borrow_mut().add_unit_id_with_target(target_hash.as_str(), unit_id.as_str());
                        }
                    }

                    self.rules_applied.insert(rule_id.to_string());
                }

                status
            }
        }
    }

    pub fn filter_headers(
        &mut self,
        headers: Vec<Header>,
        response_status_code: u16,
        add_rule_ids_header: bool,
        unit_trace: Option<Rc<RefCell<UnitTrace>>>,
    ) -> Vec<Header> {
        let mut filters = Vec::new();

        for trace in &self.rule_traces {
            if trace.on_response_status_codes.is_empty() {
                self.rules_applied.insert(trace.id.clone());
                continue;
            }

            if !trace.exclude_response_status_codes && trace.on_response_status_codes.contains(&response_status_code) {
                self.rules_applied.insert(trace.id.clone());
                continue;
            }

            if trace.exclude_response_status_codes && !trace.on_response_status_codes.contains(&response_status_code) {
                self.rules_applied.insert(trace.id.clone());
            }
        }

        for filter in self.header_filters.as_slice() {
            if !filter.on_response_status_codes.is_empty() {
                if !filter.exclude_response_status_codes && !filter.on_response_status_codes.contains(&response_status_code) {
                    continue;
                }

                if filter.exclude_response_status_codes && filter.on_response_status_codes.contains(&response_status_code) {
                    continue;
                }
            }

            filters.push(filter.filter.clone());

            if let Some(rule_id) = filter.rule_id.as_ref() {
                self.rules_applied.insert(rule_id.clone());
            }
        }

        let mut new_headers = match FilterHeaderAction::new(filters) {
            None => headers,
            Some(filter_action) => filter_action.filter(headers, unit_trace.clone()),
        };

        if let Some(trace) = unit_trace {
            trace.borrow_mut().rule_ids_applied.extend(self.get_applied_rule_ids().clone());
        }

        if add_rule_ids_header {
            new_headers.push(Header {
                name: "X-RedirectionIo-RuleIds".to_string(),
                value: self.get_applied_rule_ids().iter().cloned().collect::<Vec<String>>().join(";"),
            });
        }

        new_headers
    }

    pub fn create_filter_body(
        &mut self,
        response_status_code: u16,
        headers: &[Header],
        unit_trace: Option<Rc<RefCell<UnitTrace>>>,
    ) -> Option<FilterBodyAction> {
        let mut filters = Vec::new();
        for filter in self.body_filters.as_slice() {
            if !filter.on_response_status_codes.is_empty() {
                if !filter.exclude_response_status_codes && !filter.on_response_status_codes.contains(&response_status_code) {
                    continue;
                }

                if filter.exclude_response_status_codes && filter.on_response_status_codes.contains(&response_status_code) {
                    continue;
                }
            }

            if let Some(rule_id) = filter.rule_id.as_ref() {
                self.rules_applied.insert(rule_id.clone());
            }

            filters.push(filter.filter.clone());
        }

        let body_filter = FilterBodyAction::new(filters, headers, unit_trace);

        if body_filter.is_empty() { None } else { Some(body_filter) }
    }

    pub fn should_log_request(
        &mut self,
        allow_log_config: bool,
        response_status_code: u16,
        unit_trace: Option<Rc<RefCell<UnitTrace>>>,
    ) -> bool {
        match self.log_override.as_ref() {
            None => allow_log_config,
            Some(log_override) => {
                let (allow_log, rule_applied_id, handled) = log_override.get_log_override(response_status_code);

                if handled {
                    if let (Some(trace), Some(unit_id)) = (unit_trace, &log_override.unit_id) {
                        trace.borrow_mut().add_unit_id_with_target("configuration::log", unit_id);
                    }
                }

                if let Some(rule_id) = rule_applied_id {
                    self.rules_applied.insert(rule_id);
                }

                allow_log.unwrap_or(allow_log_config)
            }
        }
    }
}
