use crate::regex_radix_tree::{Storage, Item};
use crate::router::{RouteData, RequestMatcher, Route};
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct MatcherTreeStorage<T: RouteData, M: RequestMatcher<T> + Default + Clone> {
    matcher: M,
    regex: String,
    phantom: PhantomData<T>,
}

pub trait ItemRoute<T: RouteData>: Item {
    fn route(&self) -> Route<T>;
}

impl<T: RouteData, S: ItemRoute<T>, M: RequestMatcher<T> + Default + Clone + 'static> Storage<S> for MatcherTreeStorage<T, M> {
    fn push(&mut self, item: S) {
        self.matcher.insert(item.route());
    }

    fn remove(&mut self, id: &str) {
        self.matcher.remove(id);
    }

    fn len(&self) -> usize {
        self.matcher.len()
    }

    fn is_empty(&self) -> bool {
        self.matcher.is_empty()
    }

    fn new(regex: &str) -> Self {
        MatcherTreeStorage {
            matcher: M::default(),
            regex: regex.to_string(),
            phantom: PhantomData
        }
    }
}

