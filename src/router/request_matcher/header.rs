use crate::router::request_matcher::{PathAndQueryMatcher, RequestMatcher};
use crate::router::{Route, RouteData, RouteHeaderKind, Trace};
use http::Request;
use regex::Regex;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

#[derive(Debug, Clone)]
pub struct HeaderMatcher<T: RouteData> {
    any_header: Box<dyn RequestMatcher<T>>,
    conditions: BTreeSet<HeaderCondition>,
    condition_groups: BTreeMap<BTreeSet<HeaderCondition>, Box<dyn RequestMatcher<T>>>,
    count: usize,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
enum ValueCondition {
    IsDefined,
    IsNotDefined,
    Equals(String),
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
                RouteHeaderKind::Equals(str) => ValueCondition::Equals(str.clone()),
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

    fn match_request(&self, request: &Request<()>) -> Vec<&Route<T>> {
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

    fn trace(&self, request: &Request<()>) -> Vec<Trace<T>> {
        let mut traces = self.any_header.trace(request);
        let mut execute_conditions = BTreeMap::new();

        for (conditions, matcher) in &self.condition_groups {
            let mut matched = true;

            for condition in conditions {
                match execute_conditions.get(condition) {
                    None => {
                        // Execute condition
                        matched = matched && condition.condition.match_value(request, condition.header_name.as_str());

                        // Save result
                        execute_conditions.insert(condition.clone(), matched);

                        traces.push(Trace::new(
                            format!("Header condition on {}: {}", condition.header_name, condition.condition.format()),
                            matched,
                            true,
                            0,
                            Vec::new(),
                            Vec::new(),
                        ));

                        if !matched {
                            break;
                        }
                    }
                    Some(result) => {
                        matched = matched && *result;

                        if !matched {
                            break;
                        }
                    }
                }
            }

            if matched {
                traces.push(Trace::new(
                    "Header condition group result".to_string(),
                    matched,
                    true,
                    0,
                    matcher.trace(request),
                    Vec::new(),
                ));
            } else {
                traces.push(Trace::new(
                    "Header condition group result".to_string(),
                    matched,
                    true,
                    0,
                    Vec::new(),
                    Vec::new(),
                ));
            }
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
        Box::new(PathAndQueryMatcher::default())
    }
}

impl ValueCondition {
    pub fn match_value(&self, request: &Request<()>, name: &str) -> bool {
        match self {
            ValueCondition::IsNotDefined => !request.headers().contains_key(name),
            ValueCondition::IsDefined => request.headers().contains_key(name),
            ValueCondition::Equals(str) => {
                let values = request.headers().get_all(name);
                let mut result = false;

                for value in values {
                    result = result || value == str;
                }

                result
            }
            ValueCondition::IsNotEqualTo(str) => {
                let values = request.headers().get_all(name);
                let mut result = true;

                for value in values {
                    result = result && value != str;
                }

                result
            }
            ValueCondition::Contains(str) => {
                let values = request.headers().get_all(name);
                let mut result = false;

                for value in values {
                    result = result
                        || match value.to_str() {
                            Ok(value_str) => value_str.contains(str.as_str()),
                            _ => false,
                        };
                }

                result
            }
            ValueCondition::DoesNotContain(str) => {
                let values = request.headers().get_all(name);
                let mut result = true;

                for value in values {
                    result = result
                        && match value.to_str() {
                            Ok(value_str) => !value_str.contains(str.as_str()),
                            _ => true,
                        };
                }

                result
            }
            ValueCondition::EndsWith(str) => {
                let values = request.headers().get_all(name);
                let mut result = false;

                for value in values {
                    result = result
                        || match value.to_str() {
                            Ok(value_str) => value_str.ends_with(str.as_str()),
                            _ => false,
                        };
                }

                result
            }
            ValueCondition::StartsWith(str) => {
                let values = request.headers().get_all(name);
                let mut result = false;

                for value in values {
                    result = result
                        || match value.to_str() {
                            Ok(value_str) => value_str.starts_with(str.as_str()),
                            _ => false,
                        };
                }

                result
            }
            ValueCondition::MatchRegex(regex_string) => match Regex::new(regex_string.as_str()) {
                Err(_) => false,
                Ok(regex) => {
                    let values = request.headers().get_all(name);
                    let mut result = false;

                    for header_value in values {
                        match header_value.to_str() {
                            Err(_) => continue,
                            Ok(header_value_str) => result = result || regex.is_match(header_value_str),
                        }
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
            ValueCondition::Equals(str) => format!("equals {}", str),
            ValueCondition::IsNotEqualTo(str) => format!("is not equal to {}", str),
            ValueCondition::Contains(str) => format!("contains {}", str),
            ValueCondition::DoesNotContain(str) => format!("does not contain {}", str),
            ValueCondition::EndsWith(str) => format!("ends with {}", str),
            ValueCondition::StartsWith(str) => format!("starts with {}", str),
            ValueCondition::MatchRegex(str) => format!("match regex {}", str),
        }
    }
}
