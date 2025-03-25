use crate::api::{BodyFilter, DateTimeConstraint, Example, HeaderFilter, IpConstraint, Marker, Source, Variable};
use crate::http::Request;
use crate::marker::{Marker as RouteMarker, MarkerString, StaticOrDynamic, Transform};
use crate::router::{IntoRoute, Route, RouteDateTime, RouteHeader, RouteHeaderKind, RouteIp, RouteTime, RouteWeekday};
use crate::router_config::RouterConfig;
use cidr::AnyIpCidr;
use percent_encoding::{AsciiSet, CONTROLS, utf8_percent_encode};
use serde::{Deserialize, Serialize};
use serde_json::from_str as json_decode;
use std::cmp::Ordering;
use std::collections::HashMap;

const SIMPLE_ENCODE_SET: &AsciiSet = CONTROLS;
const URL_ENCODE_SET: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'#').add(b'<').add(b'>');
const QUERY_ENCODE_SET: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'#').add(b'<').add(b'>').add(b'+');

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Rule {
    pub id: String,
    pub source: Source,
    pub target: Option<String>,
    #[serde(alias = "redirect_code")]
    pub status_code: Option<u16>,
    pub rank: u16,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub markers: Vec<Marker>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub variables: Vec<Variable>,
    pub body_filters: Option<Vec<BodyFilter>>,
    pub header_filters: Option<Vec<HeaderFilter>>,
    pub log_override: Option<bool>,
    pub reset: Option<bool>,
    pub stop: Option<bool>,
    pub examples: Option<Vec<Example>>,
    pub redirect_unit_id: Option<String>,
    pub configuration_log_unit_id: Option<String>,
    pub configuration_reset_unit_id: Option<String>,
    pub target_hash: Option<String>,
}

impl Ord for Rule {
    fn cmp(&self, other: &Self) -> Ordering {
        let order_on_rank = other.rank.cmp(&self.rank);

        if order_on_rank != Ordering::Equal {
            return order_on_rank;
        }

        other.id.cmp(&self.id)
    }
}

impl PartialOrd for Rule {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Rule {
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank && self.id == other.id
    }
}

impl Eq for Rule {}

impl Rule {
    pub fn from_json(rule_str: &str) -> Option<Rule> {
        let rule_result = json_decode(rule_str);

        if rule_result.is_err() {
            log::error!("Unable to create rule from string {}: {}", rule_str, rule_result.err().unwrap());

            return None;
        }

        Some(rule_result.unwrap())
    }

    pub fn variables(&self, markers_captured: &HashMap<String, String>, request: &Request) -> Vec<(String, String)> {
        let mut variables = Vec::new();
        let mut input = HashMap::new();

        for (name, value) in markers_captured {
            match self.get_marker(name.as_str()) {
                None => {
                    input.insert(name.clone(), value.clone());
                }
                Some(m) => {
                    input.insert(name.clone(), m.transform(value.clone()));
                }
            }
        }

        // Clone markers capture for bc break
        if self.variables.is_empty() {
            for (name, value) in &input {
                variables.push((name.clone(), value.clone()));
            }
        } else {
            for variable in &self.variables {
                variables.push((variable.name.clone(), variable.get_value(&input, request)));
            }
        }

        variables.sort_by(|(key_a, _), (key_b, _)| key_b.len().cmp(&key_a.len()));

        variables
    }

    fn get_marker(&self, name: &str) -> Option<&Marker> {
        self.markers.iter().find(|m| m.name.as_str() == name)
    }

    fn markers(&self) -> Vec<RouteMarker> {
        let mut markers = Vec::new();

        for marker in &self.markers {
            let regex = utf8_percent_encode(marker.regex.as_str(), SIMPLE_ENCODE_SET).to_string();

            markers.push(RouteMarker::new(marker.name.clone(), regex));
        }

        markers
    }

    fn route_ips(&self) -> Option<Vec<RouteIp>> {
        match &self.source.ips {
            None => None,
            Some(ips) => {
                let mut route_ips = Vec::new();

                for ip in ips {
                    match ip {
                        IpConstraint::InRange(range) => match range.parse::<AnyIpCidr>() {
                            Ok(cidr) => route_ips.push(RouteIp::InRange(cidr)),
                            Err(err) => {
                                log::error!("cannot parse cidr {}: {}", range, err);
                            }
                        },
                        IpConstraint::NotInRange(range) => match range.parse::<AnyIpCidr>() {
                            Ok(cidr) => route_ips.push(RouteIp::NotInRange(cidr)),
                            Err(err) => {
                                log::error!("cannot parse cidr {}: {}", range, err);
                            }
                        },
                    }
                }

                if route_ips.is_empty() { None } else { Some(route_ips) }
            }
        }
    }

    fn route_datetimes(&self) -> Option<Vec<RouteDateTime>> {
        let mut route_datetimes = Vec::new();

        if let Some(source_datetimes) = self.source.datetime.as_ref() {
            for range in source_datetimes {
                let DateTimeConstraint(source_start, source_end) = range;
                route_datetimes.push(RouteDateTime::from_range(source_start, source_end));
            }
        }

        if route_datetimes.is_empty() { None } else { Some(route_datetimes) }
    }

    fn route_times(&self) -> Option<Vec<RouteTime>> {
        let mut route_times = Vec::new();

        if let Some(source_times) = self.source.time.as_ref() {
            for range in source_times {
                let DateTimeConstraint(source_start, source_end) = range;
                route_times.push(RouteTime::from_range(source_start, source_end));
            }
        }

        if route_times.is_empty() { None } else { Some(route_times) }
    }

    fn route_weekdays(&self) -> Option<RouteWeekday> {
        if let Some(source_weekdays) = self.source.weekdays.as_ref() {
            return RouteWeekday::from_weekdays(source_weekdays);
        }

        None
    }

    fn path_and_query(&self, ignore_case: bool) -> StaticOrDynamic {
        let markers = self.markers();

        let query = match self.source.query.clone() {
            None => None,
            Some(source_query) => Request::build_sorted_query(source_query.as_str()),
        };

        let mut path = utf8_percent_encode(self.source.path.as_str(), URL_ENCODE_SET).to_string();

        if let Some(query_string) = query {
            let query_string_encoded = utf8_percent_encode(query_string.as_str(), QUERY_ENCODE_SET).to_string();

            path.push_str(format!("?{query_string_encoded}").as_str());
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
                        unknown => {
                            log::error!("unsupported header constraint type {}", unknown);

                            continue;
                        }
                    },
                })
            }
        }

        headers
    }
}

impl IntoRoute<Rule> for Rule {
    fn into_route(self, config: &RouterConfig) -> Route<Rule> {
        Route::new(
            self.source.methods.clone(),
            self.source.exclude_methods,
            self.source.scheme.clone(),
            self.host(config.ignore_host_case),
            self.path_and_query(config.ignore_path_and_query_case),
            self.headers(config.ignore_header_case),
            self.route_ips(),
            self.route_datetimes(),
            self.route_times(),
            self.route_weekdays(),
            self.id.clone(),
            0 - self.rank as i64,
            self,
        )
    }
}
