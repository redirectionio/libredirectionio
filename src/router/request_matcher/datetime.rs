use crate::http::Request;
use crate::router::request_matcher::{MethodMatcher, RequestMatcher};
use crate::router::route_datetime::RouteDateTime;
use crate::router::route_time::RouteTime;
use crate::router::route_weekday::RouteWeekday;
use crate::router::trace::{TraceInfo, TraceInfoDateTimeCondition};
use crate::router::{Route, RouteData, Trace};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::collections::BTreeSet;

#[derive(Debug, Clone)]
pub struct DateTimeMatcher<T: RouteData> {
    any_datetime: Box<dyn RequestMatcher<T>>,
    conditions: BTreeSet<DateTimeCondition>,
    condition_groups: BTreeMap<BTreeSet<DateTimeCondition>, Box<dyn RequestMatcher<T>>>,
    count: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type", content = "date_time_type")]
pub enum DateTimeCondition {
    DateTimeRange(Vec<RouteDateTime>),
    TimeRange(Vec<RouteTime>),
    Weekdays(RouteWeekday),
}

impl<T: RouteData> RequestMatcher<T> for DateTimeMatcher<T> {
    fn insert(&mut self, route: Route<T>) {
        self.count += 1;

        let mut condition_group = BTreeSet::new();
        let mut conditions = BTreeSet::new();

        match route.datetime() {
            Some(route_datetime) => {
                let condition = DateTimeCondition::DateTimeRange(route_datetime.clone());
                condition_group.insert(condition.clone());
                conditions.insert(condition);
            }
            None => (),
        }

        match route.weekdays() {
            Some(route_weekdays) => {
                let condition = DateTimeCondition::Weekdays(route_weekdays.clone());
                condition_group.insert(condition.clone());
                conditions.insert(condition);
            }
            None => (),
        }

        match route.time() {
            Some(route_time) => {
                let condition = DateTimeCondition::TimeRange(route_time.clone());
                condition_group.insert(condition.clone());
                conditions.insert(condition);
            }
            None => (),
        }

        if conditions.is_empty() {
            self.any_datetime.insert(route);

            return;
        }

        self.conditions.extend(conditions);

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

        removed = removed || self.any_datetime.remove(id);

        if removed {
            self.count -= 1;
        }

        removed
    }

    fn match_request(&self, request: &Request) -> Vec<&Route<T>> {
        let mut rules = self.any_datetime.match_request(request);
        let mut execute_conditions = BTreeMap::new();

        if let Some(datetime) = request.created_at.as_ref() {
            'group: for (conditions, matcher) in &self.condition_groups {
                for condition in conditions {
                    match execute_conditions.get(condition) {
                        None => {
                            // Execute condition
                            let result = condition.match_value(datetime);

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
        }

        rules
    }

    fn trace(&self, request: &Request) -> Vec<Trace<T>> {
        let mut traces = self.any_datetime.trace(request);
        let mut execute_conditions = BTreeMap::new();

        if let Some(datetime) = request.created_at.as_ref() {
            for (conditions, matcher) in &self.condition_groups {
                let mut matched = true;
                let mut executed = true;
                let mut traces_info_datetime = Vec::new();

                for condition in conditions {
                    match execute_conditions.get(condition) {
                        None => {
                            // Execute condition
                            let result = condition.match_value(datetime);
                            matched = matched && result;

                            // Save result (only if executed to mimic cache behavior)
                            if executed {
                                execute_conditions.insert(condition.clone(), matched);
                            }

                            traces_info_datetime.push(TraceInfoDateTimeCondition {
                                result: if executed { Some(result) } else { None },
                                condition: condition.clone(),
                                against: datetime.clone(),
                                cached: false,
                            });

                            executed = matched;
                        }
                        Some(result) => {
                            matched = matched && *result;

                            traces_info_datetime.push(TraceInfoDateTimeCondition {
                                result: if executed { Some(*result) } else { None },
                                condition: condition.clone(),
                                against: datetime.clone(),
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
                    TraceInfo::DateTimeGroup {
                        conditions: traces_info_datetime,
                    },
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

        self.any_datetime.cache(new_limit, level)
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

impl<T: RouteData> Default for DateTimeMatcher<T> {
    fn default() -> Self {
        DateTimeMatcher {
            any_datetime: DateTimeMatcher::create_sub_matcher(),
            conditions: BTreeSet::new(),
            condition_groups: BTreeMap::new(),
            count: 0,
        }
    }
}

impl<T: RouteData> DateTimeMatcher<T> {
    pub fn create_sub_matcher() -> Box<dyn RequestMatcher<T>> {
        Box::<MethodMatcher<T>>::default()
    }
}

impl DateTimeCondition {
    pub fn match_value(&self, datetime: &DateTime<Utc>) -> bool {
        match self {
            DateTimeCondition::DateTimeRange(route_date_time) =>  {
                for range in route_date_time {
                    if range.match_datetime(datetime) {
                        return true;
                    }
                }
                return false;
            },
            DateTimeCondition::TimeRange(route_time) => {
                for range in route_time {
                    if range.match_datetime(datetime) {
                        return true;
                    }
                }
                return false;
            },
            DateTimeCondition::Weekdays(route_weekday) => route_weekday.match_datetime(datetime),
        }
    }
}
