use serde::{Deserialize, Serialize};

use crate::{api::VariableValue, http::Header};

const MAX_TAG_LENGTH: usize = 64;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LogTags {
    pub tags: Vec<String>,
    pub rule_id: Option<String>,
    pub on_response_status_codes: Vec<u16>,
    pub exclude_response_status_codes: bool,
    pub unit_id: Option<String>,
    // Set while `tags` still references a response header variable, see `resolve_response_headers`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub variables: Vec<(String, VariableValue)>,
}

impl LogTags {
    /// Done when filtering the headers, so the tags see the backend headers before a rule drops them.
    pub fn resolve_response_headers(&mut self, headers: &[Header]) {
        if self.variables.is_empty() {
            return;
        }

        self.tags = Self::resolve(&self.tags, &VariableValue::resolve_response_headers(&self.variables, headers));
        self.variables.clear();
    }

    pub fn resolved_tags(&self) -> Vec<String> {
        if self.variables.is_empty() {
            self.tags.clone()
        } else {
            Self::resolve(&self.tags, &self.variables)
        }
    }

    pub fn applies_to(&self, response_status_code: u16) -> bool {
        if self.on_response_status_codes.is_empty() {
            return true;
        }

        self.exclude_response_status_codes != self.on_response_status_codes.contains(&response_status_code)
    }

    /// Replaces the variables of each tag, then normalizes it. A tag referencing a variable
    /// that resolves to nothing is dropped rather than logged half filled (`lang:` for `lang:@lang`).
    pub fn resolve(tags: &[String], variables: &[(String, VariableValue)]) -> Vec<String> {
        tags.iter()
            .filter_map(|tag| replace_variables(tag, variables))
            .map(|tag| normalize(&tag))
            .filter(|tag| !tag.is_empty())
            .collect()
    }
}

// Same replacement order as StaticOrDynamic::replace, which the other actions go through.
fn replace_variables(tag: &str, variables: &[(String, VariableValue)]) -> Option<String> {
    let mut tag = tag.to_string();

    for (name, value) in variables {
        let reference = format!("@{name}");

        if !tag.contains(reference.as_str()) {
            continue;
        }

        let replacement = match value {
            VariableValue::Value(v) => v.as_str(),
            VariableValue::HtmlFilter { default: Some(v), .. } | VariableValue::ResponseHeader { default: Some(v), .. } => v.as_str(),
            VariableValue::HtmlFilter { default: None, .. } | VariableValue::ResponseHeader { default: None, .. } => "",
        };

        if replacement.is_empty() {
            return None;
        }

        tag = tag.replace(reference.as_str(), replacement);
    }

    // An "@" left over is a reference to a variable the rule does not define.
    if tag.contains('@') { None } else { Some(tag) }
}

/// Keeps letters, digits, "_", ":", "." and "-", the characters the manager accepts in a
/// tag; anything else (spaces, "/", accents...) becomes a single "-".
fn normalize(tag: &str) -> String {
    let mut normalized = String::with_capacity(tag.len());

    for c in tag.trim().chars() {
        if c.is_ascii_alphanumeric() || matches!(c, '_' | ':' | '.' | '-') {
            normalized.push(c);
        } else if !normalized.ends_with('-') {
            normalized.push('-');
        }
    }

    normalized.truncate(MAX_TAG_LENGTH);

    normalized.trim_matches('-').to_string()
}

#[cfg(test)]
mod tests {
    use super::LogTags;
    use crate::{api::VariableValue, http::Header};

    fn variables() -> Vec<(String, VariableValue)> {
        vec![
            ("lang".to_string(), VariableValue::Value("fr".to_string())),
            ("agent".to_string(), VariableValue::Value("Mozilla/5.0 (X11)".to_string())),
            ("empty".to_string(), VariableValue::Value(String::new())),
        ]
    }

    fn resolve(tags: &[&str]) -> Vec<String> {
        LogTags::resolve(&tags.iter().map(|t| t.to_string()).collect::<Vec<_>>(), &variables())
    }

    #[test]
    fn static_tags_are_kept() {
        assert_eq!(resolve(&["blog", "campaign:summer"]), vec!["blog", "campaign:summer"]);
    }

    #[test]
    fn variables_are_replaced() {
        assert_eq!(resolve(&["@lang", "lang:@lang"]), vec!["fr", "lang:fr"]);
    }

    #[test]
    fn values_are_normalized() {
        assert_eq!(resolve(&["ua:@agent"]), vec!["ua:Mozilla-5.0-X11"]);
        assert_eq!(resolve(&["a b / c"]), vec!["a-b-c"]);
        assert_eq!(resolve(&[&"x".repeat(100)]), vec!["x".repeat(64)]);
    }

    #[test]
    fn tags_with_an_empty_or_unknown_variable_are_dropped() {
        assert!(resolve(&["lang:@empty", "@unknown", "   ", "///"]).is_empty());
    }

    fn route_tags(default: Option<&str>) -> LogTags {
        LogTags {
            tags: vec!["route:@route".to_string(), "lang:@lang".to_string()],
            rule_id: None,
            on_response_status_codes: Vec::new(),
            exclude_response_status_codes: false,
            unit_id: None,
            variables: vec![
                (
                    "route".to_string(),
                    VariableValue::ResponseHeader {
                        name: "X-Route".to_string(),
                        default: default.map(str::to_string),
                        transformers: Vec::new(),
                    },
                ),
                ("lang".to_string(), VariableValue::Value("fr".to_string())),
            ],
        }
    }

    #[test]
    fn response_header_variables_are_resolved_from_the_response_headers() {
        let mut log_tags = route_tags(None);
        log_tags.resolve_response_headers(&[Header {
            name: "x-route".to_string(),
            value: "app_pricing".to_string(),
        }]);

        assert_eq!(log_tags.resolved_tags(), vec!["route:app_pricing", "lang:fr"]);
        assert!(log_tags.variables.is_empty());
    }

    #[test]
    fn response_header_variables_fall_back_to_their_default() {
        assert_eq!(route_tags(Some("none")).resolved_tags(), vec!["route:none", "lang:fr"]);
        assert_eq!(route_tags(None).resolved_tags(), vec!["lang:fr"]);
    }
}
