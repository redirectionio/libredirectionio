use crate::router::request_matcher::{DateTimeCondition, HeaderValueCondition};
use crate::router::{Route, RouteData};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RouteTrace<T: RouteData> {
    traces: Vec<Trace<T>>,
    routes: Vec<Route<T>>,
    final_route: Option<Route<T>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Trace<T: RouteData> {
    matched: bool,
    executed: bool,
    count: u64,
    #[serde(flatten)]
    info: TraceInfo<T>,
    children: Vec<Trace<T>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum TraceInfo<T: RouteData> {
    Scheme { request: String, against: Option<String> },
    HostStatic { request: String, against: Option<String> },
    HostRegex,
    Ip { request: String, against: String },
    DateTimeGroup { conditions: Vec<TraceInfoDateTimeCondition> },
    Method { request: String, against: Option<String> },
    HeaderGroup { conditions: Vec<TraceInfoHeaderCondition> },
    PathAndQueryStatic { request: String },
    PathAndQueryRegex,
    Regex { request: String, against: String },
    Storage { routes: Vec<Route<T>> },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TraceInfoHeaderCondition {
    pub result: Option<bool>,
    pub name: String,
    pub condition: HeaderValueCondition,
    pub cached: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TraceInfoDateTimeCondition {
    pub result: Option<bool>,
    pub condition: DateTimeCondition,
    pub against: DateTime<Utc>,
    pub cached: bool,
}

impl<T: RouteData> RouteTrace<T> {
    pub fn new(traces: Vec<Trace<T>>, routes: Vec<Route<T>>, final_route: Option<Route<T>>) -> RouteTrace<T> {
        RouteTrace {
            traces,
            routes,
            final_route,
        }
    }
}

impl<T: RouteData> Trace<T> {
    pub fn new(matched: bool, executed: bool, count: u64, children: Vec<Trace<T>>, info: TraceInfo<T>) -> Trace<T> {
        Trace {
            matched,
            executed,
            count,
            info,
            children,
        }
    }

    pub fn get_routes_from_traces(traces: &[Trace<T>]) -> Vec<&Route<T>> {
        let mut routes = Vec::new();

        for trace in traces {
            if let TraceInfo::Storage { routes: routes_stored } = &trace.info {
                routes.extend(routes_stored.iter().collect::<Vec<_>>());
            }

            if !trace.children.is_empty() {
                routes.extend(Trace::get_routes_from_traces(&trace.children));
            }
        }

        routes
    }
}
