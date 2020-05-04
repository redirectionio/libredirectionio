use crate::api::{Source, Marker, BodyFilter, HeaderFilter};
use serde::{Deserialize, Serialize};
use serde_json::from_str as json_decode;
use crate::router::{Route, RouteData, Marker as RouteMarker, StaticOrDynamic, Transformer};
use std::collections::BTreeMap;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const SIMPLE_ENCODE_SET: &AsciiSet = &CONTROLS;
const QUERY_ENCODE_SET: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'#').add(b'<').add(b'>');

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Rule {
    pub id: String,
    source: Source,
    pub target: Option<String>,
    pub redirect_code: Option<u16>,
    rank: u16,
    markers: Option<Vec<Marker>>,
    pub match_on_response_status: Option<u16>,
    pub body_filters: Option<Vec<BodyFilter>>,
    pub header_filters: Option<Vec<HeaderFilter>>,
}

impl RouteData for Rule {}

impl Rule {
    pub fn from_json(rule_str: &str) -> Option<Rule> {
        let rule_result= json_decode(&rule_str);

        if rule_result.is_err() {
            error!(
                "Unable to create rule from string {}: {}",
                rule_str,
                rule_result.err().unwrap()
            );

            return None;
        }

        Some(rule_result.unwrap())
    }

    pub fn path_and_query(&self) -> StaticOrDynamic {
        let markers = match &self.markers {
            None => Vec::new(),
            Some(rule_markers) => {
                let mut markers = Vec::new();

                for marker in rule_markers {
                    let regex = utf8_percent_encode(marker.regex.as_str(), SIMPLE_ENCODE_SET).to_string();

                    markers.push(RouteMarker::new(marker.name.clone(), regex));
                }

                markers
            }
        };

        let query = match self.source.query.clone() {
            None => None,
            Some(source_query) => {
                let hash_query: BTreeMap<_, _> = url::form_urlencoded::parse(source_query.as_bytes())
                    .into_owned()
                    .collect();

                let mut query_string = "".to_string();

                for (key, value) in &hash_query {
                    query_string.push_str(&utf8_percent_encode(key, QUERY_ENCODE_SET).to_string());

                    if !value.is_empty() {
                        query_string.push_str("=");
                        query_string.push_str(&utf8_percent_encode(value, QUERY_ENCODE_SET).to_string());
                    }

                    query_string.push_str("&");
                }

                query_string.pop();

                if query_string.is_empty() {
                    None
                } else {
                    Some(query_string)
                }
            }
        };

        let mut path = utf8_percent_encode(self.source.path.as_str(), SIMPLE_ENCODE_SET).to_string();

        if let Some(query_string) = query {
            path.push_str(format!("?{}", query_string).as_str());
        }

        StaticOrDynamic::new_with_markers(path.as_str(), markers)
    }

    pub fn transformers(&self) -> Vec<Transformer> {
        match self.markers.as_ref() {
            None => Vec::new(),
            Some(markers) => {
                let mut transformers = Vec::new();

                for marker in markers {
                    let mut transforms = Vec::new();

                    match marker.transformers.as_ref() {
                        None => (),
                        Some(marker_transformers) => {
                            for marker_transformer in marker_transformers {
                                match marker_transformer.to_transform() {
                                    None => (),
                                    Some(transform) => {
                                        transforms.push(transform);
                                    }
                                }
                            }
                        }
                    }

                    transformers.push(Transformer::new(marker.name.clone(), marker.name.clone(), transforms))
                }

                transformers
            }
        }
    }

    pub fn into_route(self) -> Route<Rule> {
        let id = self.id.clone();
        let priority = 0 - self.rank as i64;

        Route::new(
            self.source.methods.clone(),
            self.source.scheme.clone(),
            self.source.host.clone(),
            self.path_and_query(),
            self,
            id,
            priority,
        )
    }
}
