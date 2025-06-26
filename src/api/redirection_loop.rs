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
                    log::warn!("cannot create request from new target: {:?} : {}", new_example, err);

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

            // If the url cannot be parsed, let's treat it as a relative Url.
            // Otherwise, we check if the corresponding domain is registered in the project.
            if let Ok(url) = Url::parse(&current_url) {
                if !project_domains.is_empty() && !project_domains.contains(&url.host_str().unwrap().to_string()) {
                    // The current url target a domain that is not registered in the project.
                    // So we consider there is no redirection loop here.
                    break;
                }
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
