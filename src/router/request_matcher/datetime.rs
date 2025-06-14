use std::{
    collections::{BTreeMap, BTreeSet, HashSet},
    sync::Arc,
};

#[cfg(feature = "dot")]
use dot_graph::{Edge, Graph, Node};
use serde::{Deserialize, Serialize};

use super::super::{
    Route, RouterConfig, Trace,
    request_matcher::PathAndQueryMatcher,
    route_datetime::RouteDateTime,
    route_time::RouteTime,
    route_weekday::RouteWeekday,
    trace::{TraceInfo, TraceInfoDateTimeCondition},
};
#[cfg(feature = "dot")]
use crate::dot::DotBuilder;
use crate::http::Request;

#[derive(Debug, Clone)]
pub struct DateTimeMatcher<T> {
    any_datetime: PathAndQueryMatcher<T>,
    conditions: BTreeSet<DateTimeCondition>,
    condition_groups: BTreeMap<BTreeSet<DateTimeCondition>, PathAndQueryMatcher<T>>,
    count: usize,
    config: Arc<RouterConfig>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type", content = "date_time_type")]
pub enum DateTimeCondition {
    DateTimeRange(Vec<RouteDateTime>),
    TimeRange(Vec<RouteTime>),
    Weekdays(RouteWeekday),
}

impl<T> DateTimeMatcher<T> {
    pub fn new(config: Arc<RouterConfig>) -> Self {
        DateTimeMatcher {
            any_datetime: PathAndQueryMatcher::new(config.clone()),
            conditions: BTreeSet::new(),
            condition_groups: BTreeMap::new(),
            count: 0,
            config,
        }
    }

    pub fn insert(&mut self, route: Arc<Route<T>>) {
        self.count += 1;

        let mut condition_group = BTreeSet::new();
        let mut route_conditions = BTreeSet::new();

        if let Some(route_datetime) = route.datetime() {
            let condition = DateTimeCondition::DateTimeRange(route_datetime.clone());
            condition_group.insert(condition.clone());
            route_conditions.insert(condition.clone());
            self.conditions.insert(condition);
        }

        if let Some(route_weekdays) = route.weekdays() {
            let condition = DateTimeCondition::Weekdays(route_weekdays.clone());
            condition_group.insert(condition.clone());
            route_conditions.insert(condition.clone());
            self.conditions.insert(condition);
        }

        if let Some(route_time) = route.time() {
            let condition = DateTimeCondition::TimeRange(route_time.clone());
            condition_group.insert(condition.clone());
            route_conditions.insert(condition.clone());
            self.conditions.insert(condition);
        }

        if route_conditions.is_empty() {
            self.any_datetime.insert(route);

            return;
        }

        if !self.condition_groups.contains_key(&condition_group) {
            self.condition_groups
                .insert(condition_group.clone(), PathAndQueryMatcher::new(self.config.clone()));
        }

        let matcher = self.condition_groups.get_mut(&condition_group).unwrap();

        matcher.insert(route)
    }

    pub fn remove(&mut self, id: &str) -> Option<Arc<Route<T>>> {
        let mut removed = self.any_datetime.remove(id);

        if removed.is_some() {
            self.count -= 1;

            return removed;
        }

        self.condition_groups.retain(|_, matcher| {
            if let Some(value) = matcher.remove(id) {
                removed = Some(value);
            }

            !matcher.is_empty()
        });

        if removed.is_some() {
            self.count -= 1;
        }

        removed
    }

    pub fn batch_remove(&mut self, ids: &HashSet<String>) -> bool {
        self.any_datetime.batch_remove(ids);

        self.condition_groups.retain(|_, matcher| {
            matcher.batch_remove(ids);

            !matcher.is_empty()
        });

        self.any_datetime.is_empty() && self.condition_groups.is_empty()
    }

    pub fn match_request(&self, request: &Request) -> Vec<Arc<Route<T>>> {
        let mut rules = self.any_datetime.match_request(request);
        let mut execute_conditions = BTreeMap::new();

        'group: for (conditions, matcher) in &self.condition_groups {
            for condition in conditions {
                match execute_conditions.get(condition) {
                    None => {
                        // Execute condition
                        let result = condition.match_value(request);

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

    pub fn trace(&self, request: &Request) -> Vec<Trace<T>> {
        let mut traces = self.any_datetime.trace(request);
        let mut execute_conditions = BTreeMap::new();

        for (conditions, matcher) in &self.condition_groups {
            let mut matched = true;
            let mut executed = true;
            let mut traces_info_datetime = Vec::new();

            for condition in conditions {
                match execute_conditions.get(condition) {
                    None => {
                        // Execute condition
                        let result = condition.match_value(request);
                        matched = matched && result;

                        // Save result (only if executed to mimic cache behavior)
                        if executed {
                            execute_conditions.insert(condition.clone(), matched);
                        }

                        traces_info_datetime.push(TraceInfoDateTimeCondition {
                            result: if executed { Some(result) } else { None },
                            condition: condition.clone(),
                            cached: false,
                        });

                        executed = matched;
                    }
                    Some(result) => {
                        matched = matched && *result;

                        traces_info_datetime.push(TraceInfoDateTimeCondition {
                            result: if executed { Some(*result) } else { None },
                            condition: condition.clone(),
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

        traces
    }

    pub fn cache(&mut self, limit: u64, level: u64) -> u64 {
        let mut new_limit = self.any_datetime.cache(limit, level);

        for matcher in self.condition_groups.values_mut() {
            new_limit = matcher.cache(new_limit, level);
        }

        new_limit
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
}

impl DateTimeCondition {
    pub fn match_value(&self, request: &Request) -> bool {
        if let Some(datetime) = request.created_at.as_ref() {
            match self {
                DateTimeCondition::DateTimeRange(route_date_time) => {
                    for range in route_date_time {
                        if range.match_datetime(datetime) {
                            return true;
                        }
                    }

                    false
                }
                DateTimeCondition::TimeRange(route_time) => {
                    for range in route_time {
                        if range.match_datetime(datetime) {
                            return true;
                        }
                    }

                    false
                }
                DateTimeCondition::Weekdays(route_weekday) => route_weekday.match_datetime(datetime),
            }
        } else {
            false
        }
    }
}

#[cfg(feature = "dot")]
impl<V> DotBuilder for DateTimeMatcher<V> {
    fn graph(&self, id: &mut u32, graph: &mut Graph) -> Option<String> {
        let node_name = format!("datetime_matcher_{}", id);
        *id += 1;

        graph.add_node(Node::new(node_name.as_str()).label("datetime matcher"));

        if let Some(key) = self.any_datetime.graph(id, graph) {
            graph.add_edge(Edge::new(&node_name, &key, "any date time"));
        }

        for (conditions, matcher) in &self.condition_groups {
            if let Some(key) = matcher.graph(id, graph) {
                graph.add_edge(Edge::new(&node_name, &key, format!("date time group {:?}", conditions).as_str()));
            }
        }

        Some(node_name)
    }
}
