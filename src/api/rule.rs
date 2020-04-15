use crate::api::{Source, Marker, BodyFilter, HeaderFilter};
use serde::{Deserialize, Serialize};
use crate::router::{Route, RouteData, Marker as RouteMarker, StaticOrDynamic};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Rule {
    id: String,
    source: Source,
    target: Option<String>,
    redirect_code: u16,
    rank: u16,
    markers: Option<Vec<Marker>>,
    match_on_response_status: Option<u16>,
    body_filters: Option<Vec<BodyFilter>>,
    header_filters: Option<Vec<HeaderFilter>>,
}

impl RouteData for Rule {}

impl Rule {
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

        let id = self.id.clone();

        Route::new(
            self.source.methods.clone(),
            self.source.scheme.clone(),
            self.source.host.clone(),
            StaticOrDynamic::new_with_markers(self.source.path.as_str(), markers),
            self,
            id,
        )
    }
}
