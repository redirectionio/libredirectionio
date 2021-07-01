use crate::router::RouterConfig;
use http::uri::PathAndQuery;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use url::form_urlencoded::parse as parse_query;

const SIMPLE_ENCODE_SET: &AsciiSet = &CONTROLS;
const QUERY_ENCODE_SET: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'#').add(b'<').add(b'>');

#[derive(Serialize, Deserialize, Debug, Clone, Hash)]
pub struct PathAndQueryWithSkipped {
    pub path_and_query: String,
    pub path_and_query_matching: Option<String>,
    pub skipped_query_params: Option<String>,
    pub original: String,
}

impl PathAndQueryWithSkipped {
    pub fn from_static(path_and_query_str: &str) -> Self {
        Self {
            path_and_query: path_and_query_str.to_string(),
            path_and_query_matching: Some(path_and_query_str.to_string()),
            original: path_and_query_str.to_string(),
            skipped_query_params: None,
        }
    }

    pub fn from_config(config: &RouterConfig, path_and_query_str: &str) -> Self {
        let url = utf8_percent_encode(
            path_and_query_str.replace(" ", "%20").replace("`", "%60").as_str(),
            SIMPLE_ENCODE_SET,
        )
        .to_string();

        if !config.ignore_marketing_query_params {
            return Self {
                path_and_query_matching: Some(if config.ignore_path_and_query_case {
                    url.to_lowercase()
                } else {
                    url.clone()
                }),
                path_and_query: url,
                original: path_and_query_str.to_string(),
                skipped_query_params: None,
            };
        }

        let path_and_query: PathAndQuery = match url.parse() {
            Ok(p) => p,
            Err(err) => {
                log::error!("cannot parse url {}, don't ignore markerting query params: {}", url, err);

                return Self {
                    path_and_query_matching: Some(if config.ignore_path_and_query_case {
                        url.to_lowercase()
                    } else {
                        url.clone()
                    }),
                    path_and_query: url,
                    original: path_and_query_str.to_string(),
                    skipped_query_params: None,
                };
            }
        };

        let mut new_path_and_query = path_and_query.path().to_string();
        let mut skipped_query_params = "".to_string();

        if let Some(query) = path_and_query.query() {
            let hash_query: BTreeMap<_, _> = parse_query(query.as_bytes()).into_owned().collect();
            let mut query_string = "".to_string();

            for (key, value) in &hash_query {
                let mut query_param = "".to_string();

                query_param.push_str(&utf8_percent_encode(key, QUERY_ENCODE_SET).to_string());

                if !value.is_empty() {
                    query_param.push('=');
                    query_param.push_str(&utf8_percent_encode(value, QUERY_ENCODE_SET).to_string());
                }

                if config.marketing_query_params.contains(key) {
                    if !skipped_query_params.is_empty() {
                        skipped_query_params.push('&')
                    }

                    skipped_query_params.push_str(query_param.as_str())
                } else {
                    if !query_string.is_empty() {
                        query_string.push('&');
                    }

                    query_string.push_str(query_param.as_str())
                }
            }

            if !query_string.is_empty() {
                new_path_and_query.push('?');
                new_path_and_query.push_str(query_string.as_str());
            }
        }

        Self {
            path_and_query_matching: Some(if config.ignore_path_and_query_case {
                new_path_and_query.to_lowercase()
            } else {
                new_path_and_query.clone()
            }),
            path_and_query: new_path_and_query,
            original: path_and_query_str.to_string(),
            skipped_query_params: if config.pass_marketing_query_params_to_target && !skipped_query_params.is_empty() {
                Some(skipped_query_params)
            } else {
                None
            },
        }
    }
}
