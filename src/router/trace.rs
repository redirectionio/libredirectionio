use super::request_matcher::{DateTimeCondition, HeaderValueCondition};
use super::route::Route;
use serde::Serialize;
use std::sync::Arc;

#[derive(Serialize, Debug, Clone)]
pub struct RouteTrace<T> {
    traces: Vec<Trace<T>>,
    routes: Vec<Arc<Route<T>>>,
    final_route: Option<Arc<Route<T>>>,
}

#[derive(Serialize, Debug, Clone)]
pub struct Trace<T> {
    pub(crate) matched: bool,
    pub(crate) executed: bool,
    pub(crate) count: u64,
    #[serde(flatten)]
    pub(crate) info: TraceInfo<T>,
    pub(crate) children: Vec<Trace<T>>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum TraceInfo<T> {
    Scheme { request: String, against: Option<String> },
    HostStatic { request: String, against: Option<String> },
    HostRegex,
    Ip { request: String, against: String },
    DateTimeGroup { conditions: Vec<TraceInfoDateTimeCondition> },
    Method { request: String, against: Option<String> },
    ExcludeMethods { request: String, against: Option<Vec<String>> },
    HeaderGroup { conditions: Vec<TraceInfoHeaderCondition> },
    PathAndQueryStatic { request: String },
    PathAndQueryRegex,
    Regex { request: String, against: String },
    Storage { routes: Vec<Arc<Route<T>>> },
}

#[derive(Serialize, Debug, Clone)]
pub struct TraceInfoHeaderCondition {
    pub result: Option<bool>,
    pub name: String,
    pub condition: HeaderValueCondition,
    pub cached: bool,
}

#[derive(Serialize, Debug, Clone)]
pub struct TraceInfoDateTimeCondition {
    pub result: Option<bool>,
    pub condition: DateTimeCondition,
    pub cached: bool,
}

impl<T> RouteTrace<T> {
    pub fn new(traces: Vec<Trace<T>>, routes: Vec<Arc<Route<T>>>, final_route: Option<Arc<Route<T>>>) -> RouteTrace<T> {
        RouteTrace {
            traces,
            routes,
            final_route,
        }
    }
}

impl<T> Trace<T> {
    pub fn new(matched: bool, executed: bool, count: u64, children: Vec<Trace<T>>, info: TraceInfo<T>) -> Trace<T> {
        Trace {
            matched,
            executed,
            count,
            info,
            children,
        }
    }

    pub fn get_routes_from_traces(traces: &[Trace<T>]) -> Vec<Arc<Route<T>>> {
        let mut routes = Vec::new();

        for trace in traces {
            if let TraceInfo::Storage { routes: routes_stored } = &trace.info {
                routes.extend(routes_stored.clone());
            }

            if !trace.children.is_empty() {
                routes.extend(Trace::get_routes_from_traces(&trace.children));
            }
        }

        routes
    }
}
