extern crate lazy_static;
extern crate regex;

use regex::Regex;
use regex::RegexBuilder;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Source {
    pub scheme: Option<String>,
    pub host: Option<String>,
    path: String,
    query: Option<String>,
    #[serde(skip)]
    sorted_query: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transformer {
    #[serde(rename = "type")]
    transformer_type: Option<String>,
    options: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Marker {
    name: String,
    regex: String,
    transformers: Vec<Transformer>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BodyFilter {
    pub action: String,
    pub value: String,
    pub element_tree: Vec<String>,
    pub x_path_matcher: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HeaderFilter {
    action: String,
    header: String,
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Rule {
    pub id: String,
    pub source: Source,
    pub target: Option<String>,
    pub redirect_code: u16,
    pub rank: u16,
    markers: Vec<Marker>,
    match_on_response_status: Option<u16>,
    pub body_filters: Option<Vec<BodyFilter>>,
    header_filters: Option<Vec<HeaderFilter>>,
    #[serde(skip)]
    pub regex: Option<String>,
    #[serde(skip)]
    pub regex_with_groups: Option<String>,
    #[serde(skip)]
    pub regex_obj: Option<Regex>,
}

impl Rule {
    pub fn compile(&mut self, cache: bool) {
        self.source.build_sorted_query();
        self.build_regex(cache);
    }

    fn build_regex(&mut self, cache: bool) {
        let mut regex_str = "".to_string();
        regex_str.push_str(&self.source.path);

        if self.source.sorted_query.is_some() {
            regex_str.push_str("?");
            regex_str.push_str(self.source.sorted_query.as_ref().unwrap());
        }

        let mut regex_str = regex::escape(&regex_str).to_string();
        let mut regex_with_group = regex_str.clone();

        for marker in &self.markers {
            let marker_regex_groups = [
                "(?P<",
                marker.name.as_str(),
                ">",
                marker.regex.as_str(),
                ")",
            ]
            .join("");
            let marker_regex_no_group = ["(?:", marker.regex.as_str(), ")"].join("");

            regex_str = regex_str.replace(
                ["@", marker.name.as_str()].join("").as_str(),
                marker_regex_no_group.as_str(),
            );

            regex_with_group = regex_with_group.replace(
                ["@", marker.name.as_str()].join("").as_str(),
                marker_regex_groups.as_str(),
            )
        }

        if cache {
            let regex_builder = RegexBuilder::new(regex_str.as_str());
            let regex_obj = regex_builder.build().expect("Cannot compile rule");
            self.regex_obj = Some(regex_obj);
        }

        self.regex = Some(regex_str);
        self.regex_with_groups = Some(regex_with_group);
    }

    pub fn is_match(&self, value: &str) -> bool {
        if self.regex_obj.is_none() && self.regex.is_none() {
            return false;
        }

        if self.regex_obj.is_none() {
            let regex = Regex::new(self.regex.as_ref().unwrap().as_str())
                .expect("Cannot compile rule regex");

            return regex.is_match(value);
        }

        return self.regex_obj.as_ref().unwrap().is_match(value);
    }
}

pub fn build_sorted_query(query: String) -> Option<String> {
    let hash_query: BTreeMap<_, _> = url::form_urlencoded::parse(query.as_bytes())
        .into_owned()
        .collect();

    let mut query_string = "".to_string();

    for (key, value) in &hash_query {
        query_string.push_str(key);
        query_string.push_str("=");
        query_string.push_str(value);
        query_string.push_str("&");
    }

    query_string.pop();

    if query_string.is_empty() {
        return None;
    }

    return Some(query_string);
}

impl Source {
    fn build_sorted_query(&mut self) {
        if self.query.is_none() {
            return;
        }

        self.sorted_query = build_sorted_query(self.query.as_ref().unwrap().clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_source_compile() {
        let mut source = Source {
            scheme: Some("http".to_string()),
            host: Some("www.test.com".to_string()),
            query: Some("c=a&b=d".to_string()),
            path: "/test".to_string(),
            sorted_query: None,
        };

        source.build_sorted_query();

        assert_ne!(None, source.sorted_query);
        assert_eq!(Some("b=d&c=a".to_string()), source.sorted_query);
    }
}
