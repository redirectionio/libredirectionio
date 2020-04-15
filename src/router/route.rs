use std::fmt::Debug;
use crate::router::marker_string::StaticOrDynamic;

pub trait RouteData: Debug + Clone + Send + Sync + 'static {}

#[derive(Debug, Clone)]
pub struct Route<T> where T: RouteData {
    handler: T,
    scheme: Option<String>,
    host: Option<String>,
    methods: Option<Vec<String>>,
    path_and_query: StaticOrDynamic,
    id: String,
}

impl<T> Route<T> where T: RouteData {
    pub fn new(methods: Option<Vec<String>>, scheme: Option<String>, host: Option<String>, path_and_query: StaticOrDynamic, handler: T, id: String) -> Route<T> {
        Route {
            handler,
            scheme,
            host,
            methods,
            path_and_query,
            id
        }
    }

    pub fn handler(&self) -> &T {
        &self.handler
    }

    pub fn host(&self) -> Option<&str> {
        Some(self.host.as_ref()?.as_str())
    }

    pub fn scheme(&self) -> Option<&str> {
        Some(self.scheme.as_ref()?.as_str())
    }

    pub fn methods(&self) -> Option<&Vec<String>> {
        self.methods.as_ref()
    }

    pub fn path_and_query(&self) -> &StaticOrDynamic {
        &self.path_and_query
    }

    pub fn id(&self) -> &str {
        self.id.as_str()
    }
}
