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
    pub transformer_type: Option<String>,
    pub options: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Marker {
    pub name: String,
    regex: String,
    pub transformers: Option<Vec<Transformer>>,
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
    pub action: String,
    pub header: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Rule {
    pub id: String,
    pub source: Source,
    pub target: Option<String>,
    pub redirect_code: u16,
    pub rank: u16,
    pub markers: Option<Vec<Marker>>,
    pub match_on_response_status: Option<u16>,
    pub body_filters: Option<Vec<BodyFilter>>,
    pub header_filters: Option<Vec<HeaderFilter>>,
    #[serde(skip)]
    pub regex: Option<String>,
    #[serde(skip)]
    pub regex_with_groups: Option<String>,
    #[serde(skip)]
    pub regex_obj: Option<Regex>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Redirect {
    #[serde(rename = "status_code")]
    pub status: u16,
    #[serde(rename = "location")]
    pub target: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RouterTrace {
    pub traces: Vec<RouterTraceItem>,
    pub rules: Vec<Rule>,
    #[serde(rename = "finalRule")]
    pub final_rule: Option<Rule>,
    pub response: Option<Redirect>,
    pub duration: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RouterTraceItem {
    pub prefix: String,
    pub matches: bool,
    #[serde(rename = "rulesEvaluated")]
    pub rules_evaluated: Vec<Rule>,
    #[serde(rename = "rulesMatched")]
    pub rules_matches: Vec<Rule>,
}

impl Rule {
    pub fn compile(&mut self, cache: bool) -> Result<(), Box<dyn std::error::Error>> {
        self.source.build_sorted_query();
        self.build_regex(cache)?;

        return Ok(());
    }

    fn build_regex(&mut self, cache: bool) -> Result<(), Box<dyn std::error::Error>> {
        let mut regex_str = "".to_string();
        regex_str.push_str(&self.source.path);

        if self.source.sorted_query.is_some() {
            regex_str.push_str("\\?");
            regex_str.push_str(regex::escape(self.source.sorted_query.as_ref().unwrap()).as_str());
        }

        let mut regex_with_group = regex_str.clone();

        if self.markers.is_some() {
            for marker in self.markers.as_ref().unwrap() {
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
        }

        let regex_matching = ["^", regex_with_group.as_str(), "$"].join("");
        let regex_builder = RegexBuilder::new(regex_matching.as_str());
        let regex_obj = regex_builder.build()?;

        if cache {
            self.regex_obj = Some(regex_obj);
        }

        self.regex = Some(regex_str);
        self.regex_with_groups = Some(regex_with_group);

        return Ok(());
    }

    pub fn is_match(&self, value: &str) -> Result<bool, regex::Error> {
        if self.regex_obj.is_none() && self.regex_with_groups.is_none() {
            return Ok(false);
        }

        if self.regex_obj.is_none() {
            let regex_matching = ["^", self.regex_with_groups.as_ref().unwrap(), "$"].join("");
            let regex = Regex::new(regex_matching.as_str())?;

            return Ok(regex.is_match(value));
        }

        return Ok(self.regex_obj.as_ref().unwrap().is_match(value));
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
