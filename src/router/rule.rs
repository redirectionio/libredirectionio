extern crate lazy_static;
extern crate regex;

use regex::Regex;
use regex::RegexBuilder;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Source {
    pub scheme: Option<String>,
    pub host: String,
    path: String,
    query: String,
    #[serde(skip)]
    sorted_query: Option<String>,
    #[serde(skip)]
    pub regex: Option<String>,
    #[serde(skip)]
    pub regex_obj: Option<Regex>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transformer {
    #[serde(rename = "type")]
    transformer_type: String,
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
    action: String,
    value: String,
    element_tree: Vec<String>,
    x_path_matcher: String,
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
    target: String,
    redirect_code: u16,
    pub rank: u16,
    markers: Vec<Marker>,
    match_on_response_status: Option<u16>,
    body_filters: Option<Vec<BodyFilter>>,
    header_filters: Option<Vec<HeaderFilter>>,
}

impl Rule {
    pub fn compile(&mut self, cache: bool) {
        self.source.compile(cache);
    }
}

impl Source {
    fn compile(&mut self, cache: bool) {
        self.build_sorted_query();
        self.build_regex(cache);
    }

    fn build_regex(&mut self, cache: bool) {
        let mut regex_str = "".to_string();
        regex_str.push_str(&self.path);

        if self.sorted_query.is_some() {
            regex_str.push_str("?");
            regex_str.push_str(self.sorted_query.as_ref().unwrap());
        }

        let regex_escaped = regex::escape(&regex_str).to_string();

        if cache {
            let regex_builder = RegexBuilder::new(regex_escaped.as_str());
            let regex_obj = regex_builder.build().expect("Cannot compile rule");
            self.regex_obj = Some(regex_obj);
        }

        self.regex = Some(regex_escaped.clone());
    }

    fn build_sorted_query(&mut self) {
        let hash_query: BTreeMap<_, _> = url::form_urlencoded::parse(self.query.as_bytes())
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
            self.sorted_query = None;

            return;
        }

        self.sorted_query = Some(query_string);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_source_compile() {
        let mut source = Source {
            scheme: Some("http".to_string()),
            host: "www.test.com".to_string(),
            query: "c=a&b=d".to_string(),
            path: "/test".to_string(),
            sorted_query: None,
            regex: None,
            regex_obj: None,
        };

        source.compile(true);

        assert_ne!(None, source.sorted_query);
        assert_eq!(Some("b=d&c=a".to_string()), source.sorted_query);
    }
}
