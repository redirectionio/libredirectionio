#[cfg(not(target_arch = "wasm32"))]
mod ffi;
mod log_override;
mod status_code_update;
#[cfg(feature = "router")]
mod trace;

use crate::action::log_override::LogOverride;
#[cfg(feature = "router")]
use crate::api::Rule;
use crate::api::{BodyFilter, HeaderFilter};
#[cfg(feature = "router")]
use crate::api::{HTMLBodyFilter, TextBodyFilter};
use crate::filter::{FilterBodyAction, FilterHeaderAction};
use crate::http::Header;
#[cfg(feature = "router")]
use crate::http::Request;
#[cfg(feature = "router")]
use crate::marker::StaticOrDynamic;
#[cfg(feature = "router")]
use crate::router::Route;
use linked_hash_set::LinkedHashSet;
use serde::{Deserialize, Serialize};
pub use status_code_update::StatusCodeUpdate;
use std::collections::HashMap;
use std::fmt::Debug;
use std::iter::FromIterator;
#[cfg(feature = "router")]
pub use trace::TraceAction;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Action {
    status_code_update: Option<StatusCodeUpdate>,
    header_filters: Vec<HeaderFilterAction>,
    body_filters: Vec<BodyFilterAction>,
    // In 3.0 remove this
    pub rule_ids: Vec<String>,
    // In 3.0 make this mandatory
    rule_traces: Option<Vec<RuleTrace>>,
    // In 3.0 make this mandatory
    pub rules_applied: Option<LinkedHashSet<String>>,
    // In 3.0 make this mandatory
    log_override: Option<LogOverride>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RuleTrace {
    id: String,
    on_response_status_codes: Vec<u16>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct UnitTrace {
    rule_ids_applied: LinkedHashSet<String>,
    unit_ids_applied: LinkedHashSet<String>,
    unit_ids_seen: LinkedHashSet<String>,
    value_computed_by_units: HashMap<String, String>,
    #[serde(skip_serializing)]
    with_target_unit_trace: WithTargetUnitTrace,
}

impl UnitTrace {
    pub fn add_unit_id(&mut self, unit_id: String) {
        self.unit_ids_applied.insert(unit_id.clone());
        self.unit_ids_seen.insert(unit_id);
    }

    pub fn add_unit_id_with_target(&mut self, target: &str, unit_id: &str) {
        self.with_target_unit_trace.add_unit_id(target, unit_id);
        // Here we don't care about squashing value, since we want all value.
        self.unit_ids_seen.insert(unit_id.to_string());
    }

    pub fn override_unit_id_with_target(&mut self, target: &str, unit_id: &str) {
        self.with_target_unit_trace.override_unit_id(target, unit_id);
        // Here we don't care about squashing value, since we want all value.
        self.unit_ids_seen.insert(unit_id.to_string());
    }

    pub fn squash_with_target_unit_traces(&mut self) {
        let with_target_unit_trace = self.with_target_unit_trace.clone();
        for unit_ids in with_target_unit_trace.unit_ids_applied_by_key.values() {
            for unit_id in unit_ids {
                self.add_unit_id(unit_id.clone());
            }
        }

        // Sort, for stability in tests
        let mut tmp = Vec::from_iter(self.unit_ids_applied.clone());
        tmp.sort();
        self.unit_ids_applied = LinkedHashSet::from_iter(tmp);

        self.with_target_unit_trace = WithTargetUnitTrace::default();
    }

    pub fn add_value_computed_by_unit(&mut self, key: &str, value: &str) {
        self.value_computed_by_units.insert(key.to_string(), value.to_string());
    }

    pub fn diff(&self, other: Vec<String>) -> LinkedHashSet<String> {
        let mut diff = LinkedHashSet::new();

        for unit_id in other {
            if !&self.unit_ids_applied.contains(&unit_id) {
                diff.insert(unit_id);
            }
        }

        diff
    }

    pub fn get_rule_ids_applied(&self) -> LinkedHashSet<String> {
        self.rule_ids_applied.clone()
    }

    pub fn rule_ids_contains(&self, rule_id: &str) -> bool {
        self.rule_ids_applied.contains(rule_id)
    }

    pub fn get_unit_ids_applied(&self) -> LinkedHashSet<String> {
        self.unit_ids_applied.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct WithTargetUnitTrace {
    unit_ids_applied_by_key: HashMap<String, LinkedHashSet<String>>,
}

impl WithTargetUnitTrace {
    fn add_unit_id(&mut self, target: &str, unit_id: &str) {
        let unit_ids = self
            .unit_ids_applied_by_key
            .entry(target.to_string())
            .or_insert_with(LinkedHashSet::new);
        unit_ids.insert(unit_id.to_string());
    }

    fn override_unit_id(&mut self, target: &str, unit_id: &str) {
        self.unit_ids_applied_by_key.remove_entry(target);
        self.add_unit_id(target, unit_id);
    }
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
            rules_applied: Some(LinkedHashSet::new()),
            log_override: None,
        }
    }
}

impl Action {
    pub fn get_applied_rule_ids(&self) -> Vec<String> {
        match &self.rules_applied {
            None => self.rule_ids.clone(),
            Some(rules) => Vec::from_iter(rules.clone()),
        }
    }

    #[cfg(feature = "router")]
    pub fn get_target(route: &Route<Rule>, request: &Request) -> Option<String> {
        let markers_captured = route.capture(request);
        let variables = route.handler().variables(&markers_captured, request);
        let rule = route.handler();

        let target = rule.target.as_ref().map(|t| {
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
        });

        target
    }

    #[cfg(feature = "router")]
    pub fn from_route_rule(route: &Route<Rule>, request: &Request) -> (Option<Action>, bool, bool, Option<String>) {
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
                    rule_id: Some(rule.id.clone()),
                });
            }
        }

        let action = Action {
            status_code_update,
            header_filters,
            body_filters,
            rule_ids: vec![rule.id.clone()],
            rule_traces: Some(vec![RuleTrace {
                on_response_status_codes: on_response_status_codes.clone(),
                id: rule.id.clone(),
            }]),
            rules_applied: Some(LinkedHashSet::new()),
            log_override: rule.log_override.map(|log_override| LogOverride {
                log_override,
                rule_id: Some(rule.id.clone()),
                on_response_status_codes: on_response_status_codes.clone(),
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
                            unit_id: self_log_override.unit_id.clone(),
                        })
                    }
                }
            }
        }
    }

    #[cfg(feature = "router")]
    pub fn from_routes_rule(mut routes: Vec<&Route<Rule>>, request: &Request, mut unit_trace: Option<&mut UnitTrace>) -> Action {
        let mut action = Action::default();

        routes.sort();

        for route in routes {
            let (action_rule_opt, reset, stop, configuration_unit_id) = Action::from_route_rule(route, request);

            if let Some(action_rule) = action_rule_opt {
                if reset {
                    if let (Some(trace), Some(unit_id)) = (unit_trace.as_deref_mut(), &configuration_unit_id) {
                        trace.add_unit_id_with_target("configuration::reset", unit_id.as_str());
                    }
                    action = action_rule;
                } else {
                    action.merge(action_rule);
                }

                if stop {
                    if let (Some(trace), Some(unit_id)) = (unit_trace.as_deref_mut(), &configuration_unit_id) {
                        trace.add_unit_id_with_target("configuration::stop", unit_id.as_str());
                    }
                    return action;
                }
            }
        }

        action
    }

    pub fn get_status_code(&mut self, response_status_code: u16, unit_trace: Option<&mut UnitTrace>) -> u16 {
        match self.status_code_update.as_ref() {
            None => 0,
            Some(status_code_update) => {
                let (status, rule_applied) = status_code_update.get_status_code(response_status_code);

                if rule_applied.is_some() {
                    if let Some(trace) = unit_trace {
                        if let Some(rule_id) = rule_applied.clone() {
                            trace.rule_ids_applied.insert(rule_id);
                        }
                        if let Some(target_hash) = status_code_update.target_hash.clone() {
                            if let Some(unit_id) = status_code_update.unit_id.clone() {
                                trace.add_unit_id_with_target(target_hash.as_str(), unit_id.as_str());
                            }
                        }
                    }

                    self.apply_rule_id(rule_applied);
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
        mut unit_trace: Option<&mut UnitTrace>,
    ) -> Vec<Header> {
        let mut filters = Vec::new();

        if let Some(self_rule_traces) = self.rule_traces.clone() {
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

            self.apply_rule_id(filter.rule_id);
        }

        let mut new_headers = match FilterHeaderAction::new(filters) {
            None => headers,
            Some(filter_action) => filter_action.filter(headers, unit_trace.as_deref_mut()),
        };

        if let Some(trace) = unit_trace {
            trace.rule_ids_applied.extend(self.get_applied_rule_ids());
        }

        if add_rule_ids_header {
            new_headers.push(Header {
                name: "X-RedirectionIo-RuleIds".to_string(),
                value: self.get_applied_rule_ids().join(";"),
            });
        }

        new_headers
    }

    pub fn create_filter_body(&mut self, response_status_code: u16, headers: &[Header]) -> Option<FilterBodyAction> {
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

        let body_filter = FilterBodyAction::new(filters, headers);

        if body_filter.is_empty() {
            None
        } else {
            Some(body_filter)
        }
    }

    pub fn should_log_request(&mut self, allow_log_config: bool, response_status_code: u16, unit_trace: Option<&mut UnitTrace>) -> bool {
        let rule_applied = self.rule_traces.is_some();

        match self.log_override.as_ref() {
            None => allow_log_config,
            Some(log_override) => {
                let (allow_log, rule_applied_id, handled) = log_override.get_log_override(response_status_code);

                if handled {
                    if let (Some(trace), Some(unit_id)) = (unit_trace, &log_override.unit_id) {
                        trace.add_unit_id_with_target("configuration::log", unit_id);
                    }
                }

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

        self.rules_applied.get_or_insert(LinkedHashSet::new()).insert(rule_id.unwrap());
    }
}
