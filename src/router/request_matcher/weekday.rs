use crate::http::Request;
use crate::router::route_weekday::RouteWeekday;
use crate::router::trace::TraceInfo;
use crate::router::{TimeMatcher, RequestMatcher, Route, RouteData, Trace};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct WeekdayMatcher<T: RouteData> {
    matchers: HashMap<RouteWeekday, Box<dyn RequestMatcher<T>>>,
    no_matcher: Box<dyn RequestMatcher<T>>,
    count: usize,
}

impl<T: RouteData> RequestMatcher<T> for WeekdayMatcher<T> {
    fn insert(&mut self, route: Route<T>) {
        self.count += 1;

        match route.weekdays() {
            Some(route_weekdays) => {
                self.matchers
                    .entry(route_weekdays.clone())
                    .or_insert_with(|| Self::create_sub_matcher())
                    .insert(route.clone());
            }
            None => {
                self.no_matcher.insert(route);
            }
        }
    }

    fn remove(&mut self, id: &str) -> bool {
        let mut removed = false;

        if self.no_matcher.remove(id) {
            self.count -= 1;

            return true;
        }

        self.matchers.retain(|_, matcher| {
            removed = removed || matcher.remove(id);

            matcher.len() > 0
        });

        if removed {
            self.count -= 1;
        }

        removed
    }

    fn match_request(&self, request: &Request) -> Vec<&Route<T>> {
        let mut routes = self.no_matcher.match_request(request);

        if let Some(datetime) = request.created_at.as_ref() {
            for (weekday_matcher, matcher) in &self.matchers {
                if weekday_matcher.match_datetime(datetime) {
                    routes.extend(matcher.match_request(request));
                }
            }
        }

        routes
    }

    fn trace(&self, request: &Request) -> Vec<Trace<T>> {
        let mut traces = self.no_matcher.trace(request);

        if let Some(datetime) = request.created_at.as_ref() {
            for (weekday_matcher, matcher) in &self.matchers {
                if weekday_matcher.match_datetime(datetime) {
                    let weekday_traces = matcher.trace(request);

                    traces.push(Trace::new(
                        true,
                        true,
                        matcher.len() as u64,
                        weekday_traces,
                        TraceInfo::Weekday {
                            request: datetime.to_string(),
                            against: weekday_matcher.to_string(),
                        },
                    ));
                } else {
                    traces.push(Trace::new(
                        false,
                        true,
                        matcher.len() as u64,
                        Vec::new(),
                        TraceInfo::Weekday {
                            request: datetime.to_string(),
                            against: weekday_matcher.to_string(),
                        },
                    ))
                }
            }
        }

        traces
    }

    fn cache(&mut self, limit: u64, level: u64) -> u64 {
        let mut new_limit = self.no_matcher.cache(limit, level);

        for matcher in self.matchers.values_mut() {
            new_limit = matcher.cache(new_limit, level);
        }

        new_limit
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

impl<T: RouteData> Default for WeekdayMatcher<T> {
    fn default() -> Self {
        WeekdayMatcher {
            matchers: HashMap::new(),
            no_matcher: Self::create_sub_matcher(),
            count: 0,
        }
    }
}

impl<T: RouteData> WeekdayMatcher<T> {
    pub fn create_sub_matcher() -> Box<dyn RequestMatcher<T>> {
        Box::<TimeMatcher<T>>::default()
    }
}
