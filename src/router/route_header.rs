use std::collections::HashMap;

use serde::Serialize;

use crate::marker::MarkerString;

#[derive(Serialize, Debug, Clone)]
pub enum RouteHeaderKind {
    IsDefined,
    IsNotDefined,
    IsEquals(String),
    IsNotEqualTo(String),
    Contains(String),
    DoesNotContain(String),
    EndsWith(String),
    StartsWith(String),
    MatchRegex(MarkerString),
}

#[derive(Serialize, Debug, Clone)]
pub struct RouteHeader {
    pub kind: RouteHeaderKind,
    pub name: String,
}

impl RouteHeader {
    pub fn capture(&self, str: &str) -> HashMap<String, String> {
        match &self.kind {
            RouteHeaderKind::MatchRegex(marker_string) => marker_string.capture(str),
            _ => HashMap::new(),
        }
    }
}
