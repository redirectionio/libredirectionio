use serde::{Deserialize, Serialize};
use crate::api::{HeaderFilter, BodyFilter, Rule};
use crate::router::Route;
use crate::router::request_matcher::PathAndQueryMatcher;
use std::collections::HashMap;
use http::Request;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Action {
    status_code_update: Option<StatusCodeUpdate>,
    header_filters: Vec<HeaderFilterAction>,
    body_filters: Vec<BodyFilterAction>,
    rule_ids: Vec<String>,
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

#[derive(Serialize, Deserialize, Debug, Clone)]
struct StatusCodeUpdate {
    status_code: u16,
    on_response_status_code: u16,
    fallback_status_code: u16,
}

impl Action {
    pub fn from_routes_rule(mut routes: Vec<&Route<Rule>>, request: &Request<()>) -> Action {
        if routes.is_empty() {
            return Action {
                status_code_update: None,
                header_filters: Vec::new(),
                body_filters: Vec::new(),
                rule_ids: Vec::new(),
            };
        }

        // Reverse order of sort
        routes.sort_by(|a, b| b.priority().cmp(&a.priority()));
        let mut rule_ids = Vec::new();
        let mut status_code_update = None;
        let mut header_filters = Vec::new();
        let mut body_filters = Vec::new();

        for route in routes {
            let rule = route.handler();

            if rule.redirect_code != 0 {
                if rule.match_on_response_status.is_some() && rule.match_on_response_status.unwrap() != 0 {
                    status_code_update = match status_code_update {
                        None => Some(StatusCodeUpdate {
                            status_code: rule.redirect_code,
                            on_response_status_code: rule.match_on_response_status.unwrap(),
                            fallback_status_code: 0,
                        }),
                        Some(status_code_update_obj) => Some(StatusCodeUpdate {
                            status_code: rule.redirect_code,
                            on_response_status_code: rule.match_on_response_status.unwrap(),
                            fallback_status_code: status_code_update_obj.fallback_status_code,
                        })
                    }
                } else {
                    status_code_update = Some(StatusCodeUpdate {
                        status_code: rule.redirect_code,
                        on_response_status_code: 0,
                        fallback_status_code: rule.redirect_code,
                    })
                }
            }

            if let Some(target) = &rule.target {
                if !target.is_empty() {
                    let path = PathAndQueryMatcher::<Rule>::request_to_path(request);
                    let parameters = route.path_and_query().capture(path.as_str());

                    header_filters.push(HeaderFilterAction{
                        filter: HeaderFilter {
                            action: "add".to_string(),
                            value: "@TODO TARGET".to_string(),
                            header: "Location".to_string(),
                        },
                        on_response_status_code: rule.match_on_response_status.unwrap_or(0),
                    })
                }
            }

            if let Some(rule_header_filters) = rule.header_filters.as_ref() {
                for filter in rule_header_filters {
                    header_filters.push(HeaderFilterAction{
                        filter: filter.clone(),
                        on_response_status_code: rule.match_on_response_status.unwrap_or(0),
                    });
                }
            }

            if let Some(rule_body_filters) = rule.body_filters.as_ref() {
                for filter in rule_body_filters {
                    body_filters.push(BodyFilterAction{
                        filter: filter.clone(),
                        on_response_status_code: rule.match_on_response_status.unwrap_or(0),
                    });
                }
            }


            rule_ids.push(route.id().to_string());
        }

        Action {
            status_code_update,
            header_filters,
            body_filters,
            rule_ids,
        }
    }

    fn get_route_parameters(route: &Route<Rule>, request: &Request<()>) {

    }
}
