extern crate lazy_static;
extern crate regex;

use regex::Regex;
use regex::RegexBuilder;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use url::percent_encoding::{utf8_percent_encode, QUERY_ENCODE_SET, SIMPLE_ENCODE_SET};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Header {
    pub key: String,
    pub value: String,
    pub markers: Option<Vec<Marker>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Source {
    pub scheme: Option<String>,
    pub host: Option<String>,
    path: String,
    query: Option<String>,
    headers: Option<Vec<Header>>,
    methods: Option<Vec<String>>,
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
    pub css_selector: Option<String>,
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
    pub static_path: Option<String>,
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
        self.build_path(cache)?;

        Ok(())
    }

    fn build_path(&mut self, cache: bool) -> Result<(), Box<dyn std::error::Error>> {
        if self.markers.is_some() && !self.markers.as_ref().unwrap().is_empty() {
            self.static_path = None;
            self.build_regex(cache)?;

            return Ok(());
        }

        self.regex = None;
        self.regex_obj = None;
        self.regex_with_groups = None;

        let mut path = utf8_percent_encode(self.source.path.as_str(), SIMPLE_ENCODE_SET).to_string();

        if self.source.sorted_query.is_some() {
            path.push_str("?");
            path.push_str(self.source.sorted_query.as_ref().unwrap());
        }

        self.static_path = Some(path);

        Ok(())
    }

    fn build_regex(&mut self, cache: bool) -> Result<(), Box<dyn std::error::Error>> {
        let mut regex_str = "".to_string();
        let path = utf8_percent_encode(self.source.path.as_str(), SIMPLE_ENCODE_SET).to_string();
        regex_str.push_str(regex::escape(path.as_str()).as_str());

        if self.source.sorted_query.is_some() {
            regex_str.push_str("\\?");
            regex_str.push_str(regex::escape(self.source.sorted_query.as_ref().unwrap()).as_str());
        }

        let mut regex_with_group = regex_str.clone();

        if self.markers.is_some() {
            self.markers
                .as_mut()
                .unwrap()
                .sort_by(|a, b| b.name.len().cmp(&a.name.len()));

            for marker in self.markers.as_ref().unwrap() {
                let marker_regex =
                    utf8_percent_encode(marker.regex.as_str(), SIMPLE_ENCODE_SET).to_string();
                let marker_regex_groups = [
                    "(?P<",
                    marker.name.as_str(),
                    ">",
                    marker_regex.as_str(),
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

        if cache {
            let regex_matching = ["^", regex_with_group.as_str(), "$"].join("");
            let regex_builder = RegexBuilder::new(regex_matching.as_str());
            let regex_obj = regex_builder.build()?;

            self.regex_obj = Some(regex_obj);
        }

        self.regex = Some(regex_str);
        self.regex_with_groups = Some(regex_with_group);

        Ok(())
    }

    pub fn is_match(&self, value: &str) -> Result<bool, regex::Error> {
        if self.static_path.is_some() {
            return Ok(self.static_path.as_ref().unwrap().clone() == value);
        }

        if self.regex_obj.is_none() && self.regex_with_groups.is_none() {
            return Ok(false);
        }

        if self.regex_obj.is_none() {
            let regex_matching = ["^", self.regex_with_groups.as_ref().unwrap(), "$"].join("");
            let regex = Regex::new(regex_matching.as_str())?;

            return Ok(regex.is_match(value));
        }

        Ok(self.regex_obj.as_ref().unwrap().is_match(value))
    }
}

pub fn build_sorted_query(query: String) -> Option<String> {
    let hash_query: BTreeMap<_, _> = url::form_urlencoded::parse(query.as_bytes())
        .into_owned()
        .collect();

    let mut query_string = "".to_string();

    for (key, value) in &hash_query {
        query_string.push_str(&utf8_percent_encode(key, QUERY_ENCODE_SET).to_string());

        if !value.is_empty() {
            query_string.push_str("=");
            query_string.push_str(&utf8_percent_encode(value, QUERY_ENCODE_SET).to_string());
        }

        query_string.push_str("&");
    }

    query_string.pop();

    if query_string.is_empty() {
        return None;
    }

    Some(query_string)
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
            headers: None,
        };

        source.build_sorted_query();

        assert_ne!(None, source.sorted_query);
        assert_eq!(Some("b=d&c=a".to_string()), source.sorted_query);
    }

    #[test]
    pub fn test_source_compile_emoji() {
        let source = Source {
            scheme: Some("http".to_string()),
            host: Some("www.test.com".to_string()),
            query: None,
            path: "/🍕".to_string(),
            sorted_query: None,
            headers: None,
        };

        let mut rule = Rule {
            match_on_response_status: None,
            body_filters: None,
            header_filters: None,
            id: "id".to_string(),
            markers: None,
            rank: 0,
            redirect_code: 302,
            static_path: None,
            regex: None,
            regex_obj: None,
            regex_with_groups: None,
            source,
            target: Some("/bar".to_string()),
        };

        let compile_result = rule.compile(true);

        assert!(compile_result.is_ok());
    }
}
