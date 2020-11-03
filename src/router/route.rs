use super::RouteHeader;
use crate::router::StaticOrDynamic;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

pub trait RouteData: Debug + Clone + Send + Sync + 'static {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Route<T: RouteData> {
    handler: T,
    scheme: Option<String>,
    host: Option<StaticOrDynamic>,
    methods: Option<Vec<String>>,
    path_and_query: StaticOrDynamic,
    headers: Vec<RouteHeader>,
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

    pub fn id(&self) -> &str {
        self.id.as_str()
    }
}
