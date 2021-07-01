use crate::api::{BodyFilter, HeaderFilter, Marker, Source};
use crate::http::Request;
use crate::router::{
    Marker as RouteMarker, MarkerString, Route, RouteData, RouteHeader, RouteHeaderKind, RouterConfig, StaticOrDynamic, Transformer,
};
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use serde::{Deserialize, Serialize};
use serde_json::from_str as json_decode;

const SIMPLE_ENCODE_SET: &AsciiSet = &CONTROLS;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Rule {
    pub id: String,
    pub source: Source,
    pub target: Option<String>,
    pub redirect_code: Option<u16>,
    rank: u16,
    markers: Option<Vec<Marker>>,
    pub body_filters: Option<Vec<BodyFilter>>,
    pub header_filters: Option<Vec<HeaderFilter>>,
}

impl RouteData for Rule {}

impl Rule {
    pub fn from_json(rule_str: &str) -> Option<Rule> {
        let rule_result = json_decode(&rule_str);

        if rule_result.is_err() {
            error!("Unable to create rule from string {}: {}", rule_str, rule_result.err().unwrap());

            return None;
        }

        Some(rule_result.unwrap())
    }

    fn markers(&self) -> Vec<RouteMarker> {
        match &self.markers {
            None => Vec::new(),
            Some(rule_markers) => {
                let mut markers = Vec::new();

                for marker in rule_markers {
                    let regex = utf8_percent_encode(marker.regex.as_str(), SIMPLE_ENCODE_SET).to_string();

                    markers.push(RouteMarker::new(marker.name.clone(), regex));
                }

                markers
            }
        }
    }

    fn path_and_query(&self, ignore_case: bool) -> StaticOrDynamic {
        let markers = self.markers();

        let query = match self.source.query.clone() {
            None => None,
            Some(source_query) => Request::build_sorted_query(source_query.as_str()),
        };

        let mut path = utf8_percent_encode(self.source.path.as_str(), SIMPLE_ENCODE_SET).to_string();

        if let Some(query_string) = query {
            path.push_str(format!("?{}", query_string).as_str());
        }

        StaticOrDynamic::new_with_markers(path.as_str(), markers, ignore_case)
    }

    fn host(&self, ignore_case: bool) -> Option<StaticOrDynamic> {
        Some(StaticOrDynamic::new_with_markers(
            self.source.host.as_ref()?.as_str(),
            self.markers(),
            ignore_case,
        ))
    }

    fn headers(&self, ignore_case: bool) -> Vec<RouteHeader> {
        let mut headers = Vec::new();

        if let Some(source_headers) = self.source.headers.as_ref() {
            for header in source_headers {
                headers.push(RouteHeader {
                    name: header.name.clone(),
                    kind: match header.kind.as_ref() {
                        "is_defined" => RouteHeaderKind::IsDefined,
                        "is_not_defined" => RouteHeaderKind::IsNotDefined,
                        "is_equals" => match &header.value {
                            None => continue,
                            Some(str) => RouteHeaderKind::IsEquals(if ignore_case { str.to_lowercase() } else { str.clone() }),
                        },
                        "is_not_equal_to" => match &header.value {
                            None => continue,
                            Some(str) => RouteHeaderKind::IsNotEqualTo(if ignore_case { str.to_lowercase() } else { str.clone() }),
                        },
                        "contains" => match &header.value {
                            None => continue,
                            Some(str) => RouteHeaderKind::Contains(if ignore_case { str.to_lowercase() } else { str.clone() }),
                        },
                        "does_not_contain" => match &header.value {
                            None => continue,
                            Some(str) => RouteHeaderKind::DoesNotContain(if ignore_case { str.to_lowercase() } else { str.clone() }),
                        },
                        "ends_with" => match &header.value {
                            None => continue,
                            Some(str) => RouteHeaderKind::EndsWith(if ignore_case { str.to_lowercase() } else { str.clone() }),
                        },
                        "starts_with" => match &header.value {
                            None => continue,
                            Some(str) => RouteHeaderKind::StartsWith(if ignore_case { str.to_lowercase() } else { str.clone() }),
                        },
                        "match_regex" => match &header.value {
                            None => continue,
                            Some(str) => match MarkerString::new(str, self.markers(), ignore_case) {
                                None => continue,
                                Some(marker) => RouteHeaderKind::MatchRegex(marker),
                            },
                        },
                        _ => continue,
                    },
                })
            }
        }

        headers
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

    pub fn into_route(self, config: &RouterConfig) -> Route<Rule> {
        Route::new(
            self.source.methods.clone(),
            self.source.scheme.clone(),
            self.host(config.ignore_host_case),
            self.path_and_query(config.ignore_path_and_query_case),
            self.headers(config.ignore_header_case),
            self.id.clone(),
            0 - self.rank as i64,
            self,
        )
    }
}
