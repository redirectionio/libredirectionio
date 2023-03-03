use crate::http::Request;
use crate::router::request_matcher::{DateTimeMatcher, RequestMatcher};
use crate::router::trace::{TraceInfo, TraceInfoHeaderCondition};
use crate::router::{Route, RouteData, RouteHeaderKind, Trace};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::collections::BTreeSet;

#[derive(Debug, Clone)]
pub struct HeaderMatcher<T: RouteData> {
    any_header: Box<dyn RequestMatcher<T>>,
    conditions: BTreeSet<HeaderCondition>,
    condition_groups: BTreeMap<BTreeSet<HeaderCondition>, Box<dyn RequestMatcher<T>>>,
    count: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type", content = "value")]
pub enum ValueCondition {
    IsDefined,
    IsNotDefined,
    IsEquals(String),
    IsNotEqualTo(String),
    Contains(String),
    DoesNotContain(String),
    EndsWith(String),
    StartsWith(String),
    MatchRegex(String),
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct HeaderCondition {
    header_name: String,
    condition: ValueCondition,
}

impl<T: RouteData> RequestMatcher<T> for HeaderMatcher<T> {
    fn insert(&mut self, route: Route<T>) {
        self.count += 1;

        if route.headers().is_empty() {
            self.any_header.insert(route);

            return;
        }

        let mut condition_group = BTreeSet::new();

        for header in route.headers() {
            let condition = match &header.kind {
                RouteHeaderKind::IsDefined => ValueCondition::IsDefined,
                RouteHeaderKind::IsNotDefined => ValueCondition::IsNotDefined,
                RouteHeaderKind::IsEquals(str) => ValueCondition::IsEquals(str.clone()),
                RouteHeaderKind::IsNotEqualTo(str) => ValueCondition::IsNotEqualTo(str.clone()),
                RouteHeaderKind::Contains(str) => ValueCondition::Contains(str.clone()),
                RouteHeaderKind::DoesNotContain(str) => ValueCondition::DoesNotContain(str.clone()),
                RouteHeaderKind::EndsWith(str) => ValueCondition::EndsWith(str.clone()),
                RouteHeaderKind::StartsWith(str) => ValueCondition::StartsWith(str.clone()),
                RouteHeaderKind::MatchRegex(marker) => ValueCondition::MatchRegex(marker.regex.clone()),
            };

            let header_condition = HeaderCondition {
                header_name: header.name.to_lowercase(),
                condition,
            };

            condition_group.insert(header_condition.clone());
            self.conditions.insert(header_condition);
        }

        if !self.condition_groups.contains_key(&condition_group) {
            self.condition_groups.insert(condition_group.clone(), Self::create_sub_matcher());
        }

        let matcher = self.condition_groups.get_mut(&condition_group).unwrap();

        matcher.insert(route)
    }

    fn remove(&mut self, id: &str) -> bool {
        let mut removed = false;

        for matcher in self.condition_groups.values_mut() {
            removed = removed || matcher.remove(id);
        }

        removed = removed || self.any_header.remove(id);

        if removed {
            self.count -= 1;
        }

        removed
    }

    fn match_request(&self, request: &Request) -> Vec<&Route<T>> {
        let mut rules = self.any_header.match_request(request);
        let mut execute_conditions = BTreeMap::new();

        'group: for (conditions, matcher) in &self.condition_groups {
            for condition in conditions {
                match execute_conditions.get(condition) {
                    None => {
                        // Execute condition
                        let result = condition.condition.match_value(request, condition.header_name.as_str());

                        // Save result
                        execute_conditions.insert(condition.clone(), result);

                        if !result {
                            continue 'group;
                        }
                    }
                    Some(result) => {
                        if !result {
                            continue 'group;
                        }
                    }
                }
            }

            rules.extend(matcher.match_request(request));
        }

        rules
    }

    fn trace(&self, request: &Request) -> Vec<Trace<T>> {
        let mut traces = self.any_header.trace(request);
        let mut execute_conditions = BTreeMap::new();

        for (conditions, matcher) in &self.condition_groups {
            let mut matched = true;
            let mut executed = true;
            let mut traces_info_header = Vec::new();

            for condition in conditions {
                match execute_conditions.get(condition) {
                    None => {
                        // Execute condition
                        let result = condition.condition.match_value(request, condition.header_name.as_str());
                        matched = matched && result;

                        // Save result (only if executed to mimic cache behavior)
                        if executed {
                            execute_conditions.insert(condition.clone(), matched);
                        }

                        traces_info_header.push(TraceInfoHeaderCondition {
                            result: if executed { Some(result) } else { None },
                            name: condition.header_name.clone(),
                            condition: condition.condition.clone(),
                            cached: false,
                        });

                        executed = matched;
                    }
                    Some(result) => {
                        matched = matched && *result;

                        traces_info_header.push(TraceInfoHeaderCondition {
                            result: if executed { Some(*result) } else { None },
                            name: condition.header_name.clone(),
                            condition: condition.condition.clone(),
                            cached: true,
                        });

                        executed = matched;
                    }
                }
            }

            traces.push(Trace::new(
                matched,
                true,
                matcher.len() as u64,
                if matched { matcher.trace(request) } else { Vec::new() },
                TraceInfo::HeaderGroup {
                    conditions: traces_info_header,
                },
            ));
        }

        traces
    }

    fn cache(&mut self, limit: u64, level: u64) -> u64 {
        let mut new_limit = limit;

        for matcher in self.condition_groups.values_mut() {
            new_limit = matcher.cache(new_limit, level);
        }

        self.any_header.cache(new_limit, level)
    }

    fn len(&self) -> usize {
        self.count
    }

    fn is_empty(&self) -> bool {
        self.count == 0
    }

    fn box_clone(&self) -> Box<dyn RequestMatcher<T>> {
        Box::new((*self).clone())
    }
}

impl<T: RouteData> Default for HeaderMatcher<T> {
    fn default() -> Self {
        HeaderMatcher {
            any_header: HeaderMatcher::create_sub_matcher(),
            conditions: BTreeSet::new(),
            condition_groups: BTreeMap::new(),
            count: 0,
        }
    }
}

impl<T: RouteData> HeaderMatcher<T> {
    pub fn create_sub_matcher() -> Box<dyn RequestMatcher<T>> {
        Box::<DateTimeMatcher<T>>::default()
    }
}

impl ValueCondition {
    pub fn match_value(&self, request: &Request, name: &str) -> bool {
        match self {
            ValueCondition::IsNotDefined => !request.header_exists(name),
            ValueCondition::IsDefined => request.header_exists(name),
            ValueCondition::IsEquals(str) => {
                let values = request.header_values(name);
                let mut result = false;

                for value in values {
                    result = result || value == str;
                }

                result
            }
            ValueCondition::IsNotEqualTo(str) => {
                let values = request.header_values(name);
                let mut result = true;

                for value in values {
                    result = result && value != str;
                }

                result
            }
            ValueCondition::Contains(str) => {
                let values = request.header_values(name);
                let mut result = false;

                for value in values {
                    result = result || value.contains(str.as_str());
                }

                result
            }
            ValueCondition::DoesNotContain(str) => {
                let values = request.header_values(name);
                let mut result = true;

                for value in values {
                    result = result && !value.contains(str.as_str());
                }

                result
            }
            ValueCondition::EndsWith(str) => {
                let values = request.header_values(name);
                let mut result = false;

                for value in values {
                    result = result || value.ends_with(str.as_str());
                }

                result
            }
            ValueCondition::StartsWith(str) => {
                let values = request.header_values(name);
                let mut result = false;

                for value in values {
                    result = result || value.starts_with(str.as_str());
                }

                result
            }
            ValueCondition::MatchRegex(regex_string) => match Regex::new(regex_string.as_str()) {
                Err(_) => false,
                Ok(regex) => {
                    let values = request.header_values(name);
                    let mut result = false;

                    for header_value in values {
                        result = result || regex.is_match(header_value);
                    }

                    result
                }
            },
        }
    }

    pub fn format(&self) -> String {
        match self {
            ValueCondition::IsDefined => "is defined".to_string(),
            ValueCondition::IsNotDefined => "is not defined".to_string(),
            ValueCondition::IsEquals(str) => format!("equals {}", str),
            ValueCondition::IsNotEqualTo(str) => format!("is not equal to {}", str),
            ValueCondition::Contains(str) => format!("contains {}", str),
            ValueCondition::DoesNotContain(str) => format!("does not contain {}", str),
            ValueCondition::EndsWith(str) => format!("ends with {}", str),
            ValueCondition::StartsWith(str) => format!("starts with {}", str),
            ValueCondition::MatchRegex(str) => format!("match regex {}", str),
        }
    }
}
