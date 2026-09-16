use serde::{Deserialize, Serialize};
use url::Url;

use super::{Example, Rule};
use crate::{action::Action, http::Request, router::Router};

const REDIRECTION_CODES: [u16; 4] = [301, 302, 307, 308];

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct RedirectionLoop {
    hops: Vec<RedirectionHop>,
    error: Option<RedirectionError>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub struct RedirectionHop {
    pub url: String,
    pub status_code: u16,
    pub method: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
enum RedirectionError {
    AtLeastOneHop,
    TooManyHops,
    Loop,
}

impl RedirectionLoop {
    pub fn from_example(router: &Router<Rule>, max_hops: u8, example: &Example, project_domains: Vec<String>) -> RedirectionLoop {
        Self::compute(router, max_hops, example, project_domains)
    }
    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    pub fn has_error_too_many_hops(&self) -> bool {
        self.error.is_some() && matches!(self.error, Some(RedirectionError::TooManyHops))
    }

    pub fn has_error_loop(&self) -> bool {
        self.error.is_some() && matches!(self.error, Some(RedirectionError::Loop))
    }

    fn compute(router: &Router<Rule>, max_hops: u8, example: &Example, project_domains: Vec<String>) -> RedirectionLoop {
        // The hosts the project answers on: its domains, or, when none is configured, the host
        // of the example when it has one. A redirection to any other host leaves the project, so
        // it is followed no further.
        let mut known_hosts = project_domains;
        if known_hosts.is_empty()
            && let Ok(url) = Url::parse(&example.url)
            && let Some(host) = url.host_str()
        {
            known_hosts.push(host.to_string());
        }

        let mut current_url = example.url.clone();
        let mut current_method = example.method.clone().unwrap_or(String::from("GET"));
        let mut error = None;

        let mut hops = vec![RedirectionHop {
            url: current_url.clone(),
            status_code: 0,
            method: current_method.clone(),
        }];

        'outer: for i in 1..=max_hops {
            let new_example = example.with_url(current_url.clone()).with_method(Some(current_method.clone()));

            let request = match Request::from_example(&router.config, &new_example) {
                Ok(request) => request,
                Err(err) => {
                    tracing::warn!("cannot create request from new target: {new_example:?} : {err}");

                    break;
                }
            };

            let routes = router.match_request(&request);
            let mut action = Action::from_routes_rule(routes, &request, None);

            let action_status_code = action.get_status_code(0, None);
            let (final_status_code, backend_status_code) = if action_status_code != 0 {
                (action_status_code, action_status_code)
            } else {
                // We call the backend and get a response code
                let backend_status_code = new_example.response_status_code.unwrap_or(200);
                let final_status_code = action.get_status_code(backend_status_code, None);
                (final_status_code, backend_status_code)
            };

            if !REDIRECTION_CODES.contains(&final_status_code) {
                break;
            }

            let headers = action.filter_headers(Vec::new(), backend_status_code, false, None);

            let mut found = false;
            for header in headers.iter() {
                if header.name.to_lowercase() == "location" {
                    current_url = join_url(current_url.as_str(), header.value.as_str());
                    found = true;
                    break;
                }
            }

            if !found {
                break;
            }

            if i > 1 {
                error = Some(RedirectionError::AtLeastOneHop);
            }

            if [301, 302].contains(&final_status_code) {
                current_method = String::from("GET");
            }

            for hop in hops.iter() {
                if hop.url == current_url && hop.method == current_method {
                    hops.push(RedirectionHop {
                        url: current_url,
                        status_code: final_status_code,
                        method: current_method,
                    });
                    error = Some(RedirectionError::Loop);
                    break 'outer;
                }
            }

            hops.push(RedirectionHop {
                url: current_url.clone(),
                status_code: final_status_code,
                method: current_method.clone(),
            });

            // A url that cannot be parsed is a relative one, on the project itself. An absolute
            // one is only followed when its host is known to be the project's: with no domain
            // configured, a target on another site used to be replayed through the rules as if it
            // were ours, and "/" redirected to "https://other.test/" was reported as a loop.
            if let Ok(url) = Url::parse(&current_url)
                && let Some(host) = url.host_str()
                && !known_hosts.iter().any(|known| known == host)
            {
                break;
            }

            if i >= max_hops {
                error = Some(RedirectionError::TooManyHops);
                break;
            }
        }

        RedirectionLoop { hops, error }
    }
}

fn join_url(base: &str, path: &str) -> String {
    let base = match Url::parse(base) {
        Ok(url) => url,
        Err(_) => return path.to_string(),
    };

    let url = match base.join(path) {
        Ok(url) => url,
        Err(_) => return path.to_string(),
    };

    url.to_string()
}

#[cfg(test)]
mod tests {
    use super::RedirectionLoop;
    use crate::{
        api::{Example, Rule},
        router::Router,
        router_config::RouterConfig,
    };

    /// A router holding one 302 redirection per (path, target) pair.
    fn router_redirecting(redirections: &[(&str, &str)]) -> Router<Rule> {
        let config: RouterConfig = serde_json::from_str(
            r#"{"ignore_host_case": false, "ignore_header_case": false, "ignore_path_and_query_case": false, "ignore_marketing_query_params": true, "marketing_query_params": [], "pass_marketing_query_params_to_target": true, "always_match_any_host": false, "ignore_query_param_order": true}"#,
        )
        .unwrap();
        let mut router = Router::<Rule>::from_config(config);

        for (i, (path, target)) in redirections.iter().enumerate() {
            let rule: Rule = serde_json::from_str(&format!(
                r#"{{"source": {{"host": "", "path": "{path}", "query": "", "scheme": "", "sampling": null, "methods": [], "headers": [], "response_status_codes": [], "ips": []}}, "id": "rule-{i}", "rank": 0, "markers": [], "body_filters": [], "header_filters": [], "target": "{target}", "redirect_code": 302, "redirect_unit_id": "redirect-{i}"}}"#
            ))
            .unwrap();
            router.insert(rule);
        }

        router
    }

    fn example(url: &str) -> Example {
        Example {
            url: url.to_string(),
            method: None,
            headers: vec![],
            datetime: None,
            ip_address: None,
            response_status_code: None,
            must_match: true,
            unit_ids_applied: None,
            response_headers: vec![],
            response_body: None,
            sampling_override: None,
        }
    }

    #[test]
    fn a_redirection_to_another_site_is_not_a_loop_when_the_project_has_no_domain() {
        let router = router_redirecting(&[("/", "https://www.other.test/")]);

        let redirection_loop = RedirectionLoop::from_example(&router, 5, &example("/"), vec![]);

        assert!(!redirection_loop.has_error());
    }

    #[test]
    fn a_redirection_to_a_domain_of_the_project_is_followed() {
        let router = router_redirecting(&[("/", "https://www.mysite.test/")]);

        let redirection_loop = RedirectionLoop::from_example(&router, 5, &example("/"), vec!["www.mysite.test".to_string()]);

        assert!(redirection_loop.has_error_loop());
    }

    #[test]
    fn the_host_of_the_example_stands_for_the_project_when_it_has_no_domain() {
        let router = router_redirecting(&[("/", "https://www.mysite.test/")]);

        let redirection_loop = RedirectionLoop::from_example(&router, 5, &example("https://www.mysite.test/"), vec![]);

        assert!(redirection_loop.has_error_loop());
    }

    #[test]
    fn a_relative_redirection_onto_itself_is_a_loop() {
        let router = router_redirecting(&[("/foo", "/foo")]);

        let redirection_loop = RedirectionLoop::from_example(&router, 5, &example("/foo"), vec![]);

        assert!(redirection_loop.has_error_loop());
    }

    #[test]
    fn a_chain_of_relative_redirections_is_followed_on_the_project_but_not_on_another_host() {
        let router = router_redirecting(&[("/", "/a"), ("/a", "/")]);
        let domains = vec!["www.mysite.test".to_string()];

        // On the project, relative or on one of its domains, the chain comes back to its start.
        assert!(RedirectionLoop::from_example(&router, 5, &example("/"), domains.clone()).has_error_loop());
        assert!(RedirectionLoop::from_example(&router, 5, &example("https://www.mysite.test/"), domains.clone()).has_error_loop());

        // On another host, the first hop already leaves the project: whatever the rules would do
        // next is not ours to replay, and the example is not on a loop.
        assert!(!RedirectionLoop::from_example(&router, 5, &example("https://www.other.test/"), domains).has_error());
    }
}
