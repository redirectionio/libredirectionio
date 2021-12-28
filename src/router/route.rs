use super::RouteHeader;
use crate::http::Request;
use crate::router::route_ip::RouteIp;
use crate::router::StaticOrDynamic;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Debug;

pub trait RouteData: Debug + Clone + Send + Sync + Ord + 'static {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Route<T: RouteData> {
    handler: T,
    scheme: Option<String>,
    host: Option<StaticOrDynamic>,
    methods: Option<Vec<String>>,
    path_and_query: StaticOrDynamic,
    headers: Vec<RouteHeader>,
    ips: Option<Vec<RouteIp>>,
    id: String,
    priority: i64,
}

impl<T: RouteData> Route<T> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        methods: Option<Vec<String>>,
        scheme: Option<String>,
        host: Option<StaticOrDynamic>,
        path_and_query: StaticOrDynamic,
        headers: Vec<RouteHeader>,
        ips: Option<Vec<RouteIp>>,
        id: String,
        priority: i64,
        handler: T,
    ) -> Route<T> {
        Route {
            handler,
            scheme,
            host,
            methods,
            path_and_query,
            headers,
            ips,
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

    pub fn priority(&self) -> i64 {
        self.priority
    }

    pub fn path_and_query(&self) -> &StaticOrDynamic {
        &self.path_and_query
    }

    pub fn ips(&self) -> Option<&Vec<RouteIp>> {
        self.ips.as_ref()
    }

    pub fn id(&self) -> &str {
        self.id.as_str()
    }

    pub fn capture(&self, request: &Request) -> HashMap<String, String> {
        let path = request.path_and_query_skipped.path_and_query.as_str();
        let mut parameters = self.path_and_query().capture(path);

        if let Some(host) = self.host() {
            if let Some(request_host) = request.host.as_ref() {
                parameters.extend(host.capture(request_host));
            }
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
}

impl<T: RouteData> Ord for Route<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.handler.cmp(&other.handler)
    }
}

impl<T: RouteData> PartialOrd for Route<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.handler.partial_cmp(&other.handler)
    }
}

impl<T: RouteData> PartialEq for Route<T> {
    fn eq(&self, other: &Self) -> bool {
        self.handler.eq(&other.handler)
    }
}

impl<T: RouteData> Eq for Route<T> {}
