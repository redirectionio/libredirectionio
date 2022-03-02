use crate::http::Request;
use crate::regex_radix_tree::{NodeItem, Storage, Trace as NodeTrace};
use crate::router::trace::TraceInfo;
use crate::router::{RequestMatcher, Route, RouteData, Trace};
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct MatcherTreeStorage<T: RouteData, S: ItemRoute<T>, M: RequestMatcher<T> + Default + Clone> {
    pub matcher: M,
    phantom_route: PhantomData<T>,
    phantom_item: PhantomData<S>,
}

pub trait ItemRoute<T: RouteData>: NodeItem {
    fn route(self) -> Route<T>;
}

impl<T: RouteData, S: ItemRoute<T>, M: RequestMatcher<T> + Default + Clone + 'static> Storage<S> for MatcherTreeStorage<T, S, M> {
    fn push(&mut self, item: S) {
        self.matcher.insert(item.route());
    }

    fn remove(&mut self, id: &str) -> bool {
        self.matcher.remove(id)
    }

    fn len(&self) -> usize {
        self.matcher.len()
    }

    fn is_empty(&self) -> bool {
        self.matcher.is_empty()
    }

    fn new(_: &str) -> Self {
        MatcherTreeStorage {
            matcher: M::default(),
            phantom_route: PhantomData,
            phantom_item: PhantomData,
        }
    }
}

impl<T: RouteData, S: ItemRoute<T>, M: RequestMatcher<T> + Default + Clone + 'static> MatcherTreeStorage<T, S, M> {
    pub fn node_trace_to_router_trace(
        value: &str,
        trace: NodeTrace<S, Self>,
        request: &Request,
        root_trace_info: Option<TraceInfo<T>>,
    ) -> Trace<T> {
        let mut children = Vec::new();

        for child in trace.children {
            children.push(Self::node_trace_to_router_trace(value, child, request, None));
        }

        if let Some(storage) = trace.storage.as_ref() {
            children.extend(storage.matcher.trace(request));
        }

        match root_trace_info {
            None => Trace::new(
                trace.matched,
                true,
                trace.count,
                children,
                TraceInfo::Regex {
                    request: value.to_string(),
                    against: trace.regex,
                },
            ),
            Some(trace_info) => Trace::new(trace.matched, true, trace.count, children, trace_info),
        }
    }
}
