use crate::api::{BodyFilter, HeaderFilter, Marker, Source};
use crate::router::{Marker as RouteMarker, PathAndQueryMatcher, Route, RouteData, RouteHeader, StaticOrDynamic, Transformer};
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use serde::{Deserialize, Serialize};
use serde_json::from_str as json_decode;

const SIMPLE_ENCODE_SET: &AsciiSet = &CONTROLS;

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

    fn path_and_query(&self) -> StaticOrDynamic {
        let markers = self.markers();

        let query = match self.source.query.clone() {
            None => None,
            Some(source_query) => PathAndQueryMatcher::<Rule>::build_sorted_query(source_query.as_str()),
        };

        let mut path = utf8_percent_encode(self.source.path.as_str(), SIMPLE_ENCODE_SET).to_string();

        if let Some(query_string) = query {
            path.push_str(format!("?{}", query_string).as_str());
        }

        StaticOrDynamic::new_with_markers(path.as_str(), markers)
    }

    fn host(&self) -> Option<StaticOrDynamic> {
        Some(StaticOrDynamic::new_with_markers(
            self.source.host.as_ref()?.as_str(),
            self.markers(),
        ))
    }

    fn headers(&self) -> Vec<RouteHeader> {
        let mut headers = Vec::new();

        if let Some(source_headers) = self.source.headers.as_ref() {
            for header in source_headers {
                headers.push(RouteHeader {
                    name: header.name.clone(),
                    value: match header.value.as_ref() {
                        None => None,
                        Some(value) => Some(StaticOrDynamic::new_with_markers(value.as_str(), self.markers())),
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

    pub fn into_route(self) -> Route<Rule> {
        Route::new(
            self.source.methods.clone(),
            self.source.scheme.clone(),
            self.host(),
            self.path_and_query(),
            self.headers(),
            self.id.clone(),
            0 - self.rank as i64,
            self,
        )
    }
}
