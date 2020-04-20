use crate::api::{Source, Marker, BodyFilter, HeaderFilter};
use serde::{Deserialize, Serialize};
use serde_json::from_str as json_decode;
use crate::router::{Route, RouteData, Marker as RouteMarker, StaticOrDynamic};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Rule {
    pub id: String,
    source: Source,
    pub target: Option<String>,
    pub redirect_code: u16,
    rank: u16,
    markers: Option<Vec<Marker>>,
    pub match_on_response_status: Option<u16>,
    pub body_filters: Option<Vec<BodyFilter>>,
    pub header_filters: Option<Vec<HeaderFilter>>,
}

impl RouteData for Rule {}

impl Rule {
    pub fn from_str(rule_str: &str) -> Option<Rule> {
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

    pub fn to_route(self) -> Route<Rule> {
        let markers = match &self.markers {
            None => Vec::new(),
            Some(rule_markers) => {
                let mut markers = Vec::new();

                for marker in rule_markers {
                    markers.push(RouteMarker::new(marker.name.clone(), marker.regex.clone()));
                }

                markers
            }
        };

        // @TODO sort query parameters
        // @TODO Encode path

        let id = self.id.clone();
        let priority = 0 - self.rank.clone() as i64;

        Route::new(
            self.source.methods.clone(),
            self.source.scheme.clone(),
            self.source.host.clone(),
            StaticOrDynamic::new_with_markers(self.source.path.as_str(), markers),
            self,
            id,
            priority,
        )
    }
}
