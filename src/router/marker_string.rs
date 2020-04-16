use std::collections::HashMap;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct Marker {
    name: String,
    regex: String,
}

#[derive(Debug, Clone)]
pub enum StaticOrDynamic {
    Static(String),
    Dynamic(MarkerString),
}

#[derive(Debug, Clone)]
pub struct MarkerString {
    pub regex: String,
    pub capture: String,
}

impl Marker {
    pub fn new(name: String, regex: String) -> Marker {
        Marker {
            name,
            regex,
        }
    }

    pub fn format(&self) -> String {
        format!("@{}", self.name)
    }
}

impl StaticOrDynamic {
    pub fn new_with_markers(str: &str, mut markers: Vec<Marker>) -> StaticOrDynamic {
        if markers.len() == 0 {
            return StaticOrDynamic::Static(str.to_string());
        }

        // Create regex string
        let mut regex = regex::escape(str);
        let mut capture = regex.clone();

        // Sort markers by length
        markers.sort_by(|a, b| b.name.len().cmp(&a.name.len()));

        // Foreach marker replace
        for marker in &markers {
            let marker_regex = format!("(?:{})", marker.regex);
            let marker_capture = format!("(?P<{}>{})", marker.name, marker.regex);

            regex = regex.replace(marker.format().as_str(), marker_regex.as_str());
            capture = capture.replace(marker.format().as_str(), marker_capture.as_str());
        }

        StaticOrDynamic::Dynamic(MarkerString {
            regex,
            capture,
        })
    }

    pub fn capture(&self, str: &str) -> HashMap<String, String> {
        match &self {
            StaticOrDynamic::Static(_) => HashMap::new(),
            StaticOrDynamic::Dynamic(marker_string) => {
                let regex = Regex::new(marker_string.capture.as_str()).expect("cannot compile regex");
                let captured = regex.captures_iter(str);

                HashMap::new()
            }
        }
    }
}
