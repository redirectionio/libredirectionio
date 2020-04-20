pub mod request_matcher;
mod route;
mod marker_string;
mod trace;
mod transformer;

pub use route::{RouteData, Route};
pub use marker_string::{StaticOrDynamic, Marker};
pub use trace::{RouteTrace, Trace};
pub use request_matcher::{PathAndQueryMatcher, SchemeMatcher, RequestMatcher};
pub use transformer::{Transform, Transformer, Camelize, Uppercase, Underscorize, Slice, Replace, Dasherize, Lowercase};

use crate::api::Rule;
use regex::Regex;
use std::time;
use url;
use url::Url;
use http::Request;

#[derive(Debug)]
pub struct Router<T: RouteData> {
    matcher: SchemeMatcher<T>,
}

impl<T: RouteData> Router<T> {
    pub fn new() -> Router<T> {
        Router {
            matcher: SchemeMatcher::new()
        }
    }

    pub fn insert(&mut self, route: Route<T>) {
        self.matcher.insert(route);
    }

    pub fn remove(&mut self, id: &str) -> Vec<Route<T>> {
        self.matcher.remove(id)
    }

    pub fn match_request(&self, request: &Request<()>) -> Vec<&Route<T>> {
        self.matcher.match_request(request)
    }

    pub fn trace_request(&self, request: &Request<()>) -> Vec<Trace<T>> {
        self.matcher.trace(request)
    }

    pub fn get_route(&self, request: &Request<()>) -> Option<&Route<T>> {
        let mut routes = self.match_request(request);

        if routes.is_empty() {
            return None;
        }

        routes.sort_by(|a, b| a.priority().cmp(&b.priority()));

        match routes.get(0) {
            None => None,
            Some(route) => {
                Some(route.clone())
            }
        }
    }

    pub fn get_trace(&self, request: &Request<()>) -> RouteTrace<T> {
        let traces = self.trace_request(request);
        let mut routes_traces = Trace::get_routes_from_traces(&traces);
        let mut routes = Vec::new();

        for &route in &routes_traces {
            routes.push(route.clone());
        }

        routes_traces.sort_by(|&a, &b| a.priority().cmp(&b.priority()));

        let final_route = match routes_traces.is_empty() {
            true => None,
            false => Some(routes_traces.first().unwrap().clone().clone())
        };

        RouteTrace::new(traces, routes, final_route)
    }

    pub fn cache(&mut self, limit: u64) {
        let mut prev_cache_limit = limit;
        let mut level = 0;

        while prev_cache_limit > 0 {
            let next_cache_limit = self.matcher.cache(prev_cache_limit, level);

            if next_cache_limit == prev_cache_limit && level > 5 {
                break;
            }

            level += 1;
            prev_cache_limit = next_cache_limit;
        }
    }

    // fn create_request(url_str: String) -> Result<Request<()>, url::ParseError> {
    //     let url = MainRouter::parse_url(url_str)?;
    //
    //     Ok(Request::builder().uri(url.to_string()).body(()).unwrap())
    // }
    //
    // fn parse_url(url_str: String) -> Result<Url, url::ParseError> {
    //     let options = url::Url::options();
    //     let base_url = Url::parse("scheme://0.0.0.0")?;
    //     let parser = options.base_url(Some(&base_url));
    //
    //     let url_obj = parser.parse(url_str.as_str())?;
    //
    //     Ok(MainRouter::sort_query(url_obj))
    // }
    //
    // fn sort_query(url_obj: Url) -> Url {
    //     if url_obj.query().is_none() {
    //         return url_obj;
    //     }
    //
    //     let mut new_url_obj = url_obj;
    //     let query_string = build_sorted_query(new_url_obj.query().unwrap().to_string());
    //
    //     match query_string {
    //         Some(query) => new_url_obj.set_query(Some(query.as_str())),
    //         None => {
    //             new_url_obj.set_query(None);
    //         }
    //     }
    //
    //     new_url_obj
    // }
    //
    // pub fn match_rules(
    //     &self,
    //     url_str: String,
    // ) -> Result<Vec<&rule::Rule>, Box<dyn std::error::Error>> {
    //     let request = MainRouter::create_request(url_str)?;
    //
    //     self.router_scheme.match_rule(&request)
    // }
    //
    // pub fn match_rule(
    //     &self,
    //     url: String,
    // ) -> Result<Option<&rule::Rule>, Box<dyn std::error::Error>> {
    //     let mut rules = self.match_rules(url)?;
    //
    //     if rules.is_empty() {
    //         return Ok(None);
    //     }
    //
    //     rules.sort_by(|a, b| a.rank.cmp(&b.rank));
    //
    //     Ok(Some(*rules.first().unwrap()))
    // }
    //
    // pub fn trace(&self, url_str: String) -> Result<rule::RouterTrace, Box<dyn std::error::Error>> {
    //     let request = MainRouter::create_request(url_str.clone())?;
    //     let traces = self.router_scheme.trace(&request)?;
    //     let start = time::Instant::now();
    //     let mut matched_rules = self.router_scheme.match_rule(&request)?;
    //     let elapsed = (start.elapsed().as_micros() as f64) / 1000.0;
    //     let mut final_rule = None;
    //
    //     if !matched_rules.is_empty() {
    //         matched_rules.sort_by(|a, b| a.rank.cmp(&b.rank));
    //         final_rule = Some((*matched_rules.first().unwrap()).clone());
    //     }
    //
    //     let mut rules = Vec::new();
    //
    //     for matched_rule in matched_rules {
    //         rules.push(matched_rule.clone());
    //     }
    //
    //     let mut redirect = None;
    //
    //     if final_rule.is_some() {
    //         let target = MainRouter::get_redirect(final_rule.as_ref().unwrap(), url_str)?;
    //
    //         redirect = Some(rule::Redirect {
    //             status: final_rule.as_ref().unwrap().redirect_code,
    //             target,
    //         });
    //     }
    //
    //     let trace = rule::RouterTrace {
    //         final_rule,
    //         traces,
    //         rules,
    //         response: redirect,
    //         duration: elapsed,
    //     };
    //
    //     Ok(trace)
    // }
    //
    // pub fn get_redirect(
    //     rule_to_redirect: &rule::Rule,
    //     url_str: String,
    // ) -> Result<String, Box<dyn std::error::Error>> {
    //     let url_object = MainRouter::parse_url(url_str)?;
    //
    //     // No markers
    //     if rule_to_redirect.static_path.is_some() {
    //         return Ok(rule_to_redirect.target.as_ref().unwrap().clone());
    //     }
    //
    //     let regex_groups_str = [
    //         "^",
    //         rule_to_redirect.regex_with_groups.as_ref().unwrap(),
    //         "$",
    //     ]
    //     .join("");
    //     let regex_groups = Regex::new(regex_groups_str.as_str())?;
    //
    //     let mut path = url_object.path().to_string();
    //
    //     if url_object.query().is_some() {
    //         let sorted_query = rule::build_sorted_query(url_object.query().unwrap().to_string());
    //
    //         if sorted_query.is_some() {
    //             path = [path.as_str(), "?", sorted_query.as_ref().unwrap().as_str()].join("");
    //         }
    //     }
    //
    //     let target_opt = rule_to_redirect.target.as_ref();
    //
    //     if target_opt.is_none() {
    //         return Ok("".to_string());
    //     }
    //
    //     let mut target = target_opt.unwrap().to_string();
    //     let mut capture_option = regex_groups.captures(path.as_str());
    //
    //     if capture_option.is_none() {
    //         capture_option = regex_groups.captures(path.as_str());
    //
    //         if capture_option.is_none() {
    //             return Ok(target);
    //         }
    //     }
    //
    //     let capture_item = capture_option.unwrap();
    //
    //     if target_opt.is_none() {
    //         return Ok(target);
    //     }
    //
    //     for named_group in regex_groups.capture_names().into_iter() {
    //         if named_group.is_none() {
    //             continue;
    //         }
    //
    //         let capture_match = capture_item.name(named_group.unwrap());
    //
    //         if capture_match.is_none() {
    //             continue;
    //         }
    //
    //         let mut marker_data = capture_match.unwrap().as_str().to_string();
    //         let marker_name = named_group.unwrap().to_string();
    //
    //         if rule_to_redirect.markers.is_some() {
    //             for marker in rule_to_redirect.markers.as_ref().unwrap() {
    //                 if marker.name == marker_name && marker.transformers.is_some() {
    //                     for transformer in marker.transformers.as_ref().unwrap() {
    //                         marker_data = transform::transform(marker_data, transformer);
    //                     }
    //                 }
    //             }
    //         }
    //
    //         target = target.replace(
    //             ["@", marker_name.as_str()].join("").as_str(),
    //             marker_data.as_str(),
    //         );
    //     }
    //
    //     Ok(target)
    // }
}
