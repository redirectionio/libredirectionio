#[cfg(feature = "dot")]
use std::sync::Arc;
use std::{cmp::Ordering, collections::HashMap, fmt::Debug};

#[cfg(feature = "dot")]
use dot_graph::{Graph, Node as GraphNode};
use serde::Serialize;

use super::{RouteHeader, route_datetime::RouteDateTime, route_ip::RouteIp, route_time::RouteTime, route_weekday::RouteWeekday};
#[cfg(feature = "dot")]
use crate::dot::DotBuilder;
use crate::{http::Request, marker::StaticOrDynamic, router::RouterConfig};

#[derive(Serialize, Debug, Clone)]
pub struct Route<T> {
    handler: T,
    scheme: Option<String>,
    host: Option<StaticOrDynamic>,
    methods: Option<Vec<String>>,
    exclude_methods: Option<bool>,
    path_and_query: StaticOrDynamic,
    headers: Vec<RouteHeader>,
    ips: Option<Vec<RouteIp>>,
    datetime: Option<Vec<RouteDateTime>>,
    time: Option<Vec<RouteTime>>,
    weekdays: Option<RouteWeekday>,
    id: String,
    priority: i64,
}

impl<T> Route<T> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        methods: Option<Vec<String>>,
        exclude_methods: Option<bool>,
        scheme: Option<String>,
        host: Option<StaticOrDynamic>,
        path_and_query: StaticOrDynamic,
        headers: Vec<RouteHeader>,
        ips: Option<Vec<RouteIp>>,
        datetime: Option<Vec<RouteDateTime>>,
        time: Option<Vec<RouteTime>>,
        weekdays: Option<RouteWeekday>,
        id: String,
        priority: i64,
        handler: T,
    ) -> Route<T> {
        Route {
            handler,
            scheme,
            host,
            methods,
            exclude_methods,
            path_and_query,
            headers,
            ips,
            datetime,
            time,
            weekdays,
            id,
            priority,
        }
    }

    pub fn handler(&self) -> &T {
        &self.handler
    }

    pub fn host(&self) -> Option<&StaticOrDynamic> {
        self.host.as_ref()
    }

    pub fn scheme(&self) -> Option<&str> {
        Some(self.scheme.as_ref()?.as_str())
    }

    pub fn headers(&self) -> &Vec<RouteHeader> {
        self.headers.as_ref()
    }

    pub fn methods(&self) -> Option<&Vec<String>> {
        self.methods.as_ref()
    }

    pub fn exclude_methods(&self) -> Option<bool> {
        self.exclude_methods
    }

    pub fn priority(&self) -> i64 {
        self.priority
    }

    pub fn path_and_query(&self) -> &StaticOrDynamic {
        &self.path_and_query
    }

    pub fn ips(&self) -> Option<&Vec<RouteIp>> {
        self.ips.as_ref()
    }

    pub fn datetime(&self) -> Option<&Vec<RouteDateTime>> {
        self.datetime.as_ref()
    }

    pub fn time(&self) -> Option<&Vec<RouteTime>> {
        self.time.as_ref()
    }

    pub fn weekdays(&self) -> Option<&RouteWeekday> {
        self.weekdays.as_ref()
    }

    pub fn id(&self) -> &str {
        self.id.as_str()
    }

    pub fn capture(&self, request: &Request) -> HashMap<String, String> {
        let path = request.path_and_query_skipped.path_and_query.as_str();
        let mut parameters = self.path_and_query().capture(path);

        if let Some(host) = self.host()
            && let Some(request_host) = request.host.as_ref()
        {
            parameters.extend(host.capture(request_host));
        }

        for header in self.headers() {
            for request_header in &request.headers {
                if request_header.name != header.name {
                    continue;
                }

                parameters.extend(header.capture(request_header.value.as_str()));
            }
        }

        parameters
    }

    pub fn compile(&self) -> u8 {
        let mut compiled = 0;

        if self.path_and_query.compile() {
            compiled += 1;
        }

        if let Some(host) = &self.host
            && host.compile()
        {
            compiled += 1;
        }

        compiled
    }
}

impl<T> PartialEq for Route<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.handler.eq(&other.handler)
    }
}

impl<T> Eq for Route<T> where T: PartialEq {}

impl<T> PartialOrd for Route<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.handler.partial_cmp(&other.handler)
    }
}

impl<T> Ord for Route<T>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.handler.cmp(&other.handler)
    }
}

pub trait IntoRoute<T> {
    fn into_route(self, config: &RouterConfig) -> Route<T>;
}

#[cfg(feature = "dot")]
impl<V> DotBuilder for Arc<Route<V>> {
    fn graph(&self, id: &mut u32, graph: &mut Graph) -> Option<String> {
        let node_name = format!("route_{}", id);
        *id += 1;

        graph.add_node(GraphNode::new(node_name.as_str()).label(self.id.as_str()));

        Some(node_name)
    }
}
