use regex::RegexBuilder;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Marker {
    name: String,
    regex: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum StaticOrDynamic {
    Static(String),
    Dynamic(MarkerString),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MarkerString {
    pub regex: String,
    pub capture: String,
    pub ignore_case: bool,
    markers: HashMap<String, String>,
}

impl Marker {
    pub fn new(name: String, regex: String) -> Marker {
        Marker { name, regex }
    }

    pub fn format(&self) -> String {
        format!("@{}", self.name)
    }
}

impl MarkerString {
    pub fn new(str: &str, mut markers: Vec<Marker>, ignore_case: bool) -> Option<MarkerString> {
        // Create regex string
        let mut regex = regex::escape(str);
        let mut capture = regex.clone();
        let mut marker_map = HashMap::new();

        // Sort markers by length
        markers.sort_by(|a, b| b.name.len().cmp(&a.name.len()));

        // Foreach marker replace
        for marker in &markers {
            let marker_regex = format!("(?:{})", marker.regex);
            let marker_capture = format!("(?P<{}>{})", marker.name, marker.regex);

            if regex.contains(marker.format().as_str()) {
                regex = regex.replace(marker.format().as_str(), marker_regex.as_str());
                capture = capture.replace(marker.format().as_str(), marker_capture.as_str());
                marker_map.insert(marker.name.clone(), marker_capture);
            }
        }

        if marker_map.is_empty() {
            return None;
        }

        Some(MarkerString {
            regex,
            capture,
            markers: marker_map,
            ignore_case,
        })
    }

    pub fn capture(&self, str: &str) -> HashMap<String, String> {
        let mut parameters = HashMap::new();
        let regex = ["^", self.capture.as_str(), "$"].join("");

        let regex_captures = match RegexBuilder::new(regex.as_str()).case_insensitive(self.ignore_case).build() {
            Err(_) => return parameters,
            Ok(regex) => regex,
        };

        let capture = match regex_captures.captures(str) {
            None => return parameters,
            Some(capture) => capture,
        };

        for named_group in regex_captures.capture_names() {
            let name = match named_group {
                None => continue,
                Some(group) => group,
            };

            let value = match capture.name(name) {
                None => continue,
                Some(matched) => matched.as_str().to_string(),
            };

            parameters.insert(name.to_string(), value);
        }

        parameters
    }
}

impl StaticOrDynamic {
    pub fn new_with_markers(str: &str, markers: Vec<Marker>, ignore_case: bool) -> StaticOrDynamic {
        if markers.is_empty() {
            if ignore_case {
                return StaticOrDynamic::Static(str.to_lowercase());
            }

            return StaticOrDynamic::Static(str.to_string());
        }

        match MarkerString::new(str, markers, ignore_case) {
            None => StaticOrDynamic::Static(if ignore_case { str.to_lowercase() } else { str.to_string() }),
            Some(marker) => StaticOrDynamic::Dynamic(marker),
        }
    }

    pub fn capture(&self, str: &str) -> HashMap<String, String> {
        match &self {
            StaticOrDynamic::Static(_) => HashMap::new(),
            StaticOrDynamic::Dynamic(marker_string) => marker_string.capture(str),
        }
    }

    pub fn replace(mut str: String, variables: &HashMap<String, String>) -> String {
        for (name, value) in variables {
            str = str.replace(format!("@{}", name).as_str(), value.as_str())
        }

        str
    }
}
