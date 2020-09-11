use http::uri::PathAndQuery;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashSet};
use url::form_urlencoded::parse as parse_query;

const SIMPLE_ENCODE_SET: &AsciiSet = &CONTROLS;
const QUERY_ENCODE_SET: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'#').add(b'<').add(b'>');

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QueryParamSkipBuilder {
    parameters: HashSet<String>,
}

impl Default for QueryParamSkipBuilder {
    fn default() -> QueryParamSkipBuilder {
        let mut parameters = HashSet::new();

        parameters.insert("utm_source".to_string());
        parameters.insert("utm_medium".to_string());
        parameters.insert("utm_campaign".to_string());
        parameters.insert("utm_term".to_string());
        parameters.insert("utm_content".to_string());

        QueryParamSkipBuilder {
            parameters,
        }
    }
}

impl QueryParamSkipBuilder {
    pub fn add_query_param(&mut self, key: &str) {
        self.parameters.insert(key.to_string());
    }

    pub fn build_query_param_skipped(&self, path_and_query_str: &str) -> PathAndQueryWithSkipped {
        let url = utf8_percent_encode(path_and_query_str.replace(" ", "%20").as_str(), SIMPLE_ENCODE_SET).to_string();
        let path_and_query: PathAndQuery = url.parse().unwrap();

        let mut new_path_and_query = path_and_query.path().to_string();
        let mut skipped_query_params = "".to_string();

        if let Some(query) = path_and_query.query() {
            let hash_query: BTreeMap<_, _> = parse_query(query.as_bytes()).into_owned().collect();
            let mut query_string= "".to_string();

            for (key, value) in &hash_query {
                let mut query_param = "".to_string();

                query_param.push_str(&utf8_percent_encode(key, QUERY_ENCODE_SET).to_string());

                if !value.is_empty() {
                    query_param.push_str("=");
                    query_param.push_str(&utf8_percent_encode(value, QUERY_ENCODE_SET).to_string());
                }

                if self.parameters.contains(key) {
                    if !skipped_query_params.is_empty() {
                        skipped_query_params.push_str("&")
                    }

                    skipped_query_params.push_str(query_param.as_str())
                } else {
                    if !query_string.is_empty() {
                        query_string.push_str("&");
                    }

                    query_string.push_str(query_param.as_str())
                }
            }

            if !query_string.is_empty() {
                new_path_and_query.push_str("?");
                new_path_and_query.push_str(query_string.as_str());
            }
        }

        PathAndQueryWithSkipped {
            original: path_and_query_str.to_string(),
            path_and_query: new_path_and_query,
            skipped_query_params: if skipped_query_params.is_empty() { None } else { Some(skipped_query_params) },
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PathAndQueryWithSkipped {
    pub original: String,
    pub path_and_query: String,
    pub skipped_query_params: Option<String>,
}
