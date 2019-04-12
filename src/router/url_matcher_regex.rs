extern crate regex;

use crate::router::rule;
use crate::router::url_matcher;
use crate::router::url_matcher::UrlMatcher;
use regex::Regex;
use url::percent_encoding::percent_decode;
use url::Url;

#[derive(Debug)]
pub struct UrlMatcherItem {
    regex: String,
    regex_obj: Option<Regex>,
    pub matcher: Box<UrlMatcher>,
}

#[derive(Debug)]
pub struct UrlMatcherRegex {
    children: Vec<UrlMatcherItem>,
    empty: Option<UrlMatcherItem>,
}

impl UrlMatcherItem {
    pub fn new(regex_str: String, matcher: Box<UrlMatcher>, cache: bool) -> UrlMatcherItem {
        let mut regex_obj = None;

        if cache {
            regex_obj = Some(Regex::new(regex_str.as_str()).expect("Cannot compile regex"));
        }

        return UrlMatcherItem {
            regex: regex_str,
            regex_obj,
            matcher,
        };
    }

    fn match_string(&self, value: &String) -> Option<&Box<UrlMatcher>> {
        if self.regex_obj.is_some() {
            let regex = self.regex_obj.as_ref().unwrap();

            if regex.is_match(value) {
                return Some(&self.matcher);
            }

            return None;
        }

        let regex = Regex::new(self.regex.as_str()).expect("Cannot compile regex");

        if regex.is_match(value) {
            return Some(&self.matcher);
        }

        return None;
    }

    fn get_rules(&self) -> Vec<&rule::Rule> {
        return self.matcher.get_rules();
    }
}

impl UrlMatcherRegex {
    pub fn new(children: Vec<UrlMatcherItem>, empty: Option<UrlMatcherItem>) -> UrlMatcherRegex {
        UrlMatcherRegex { children, empty }
    }
}

impl url_matcher::UrlMatcher for UrlMatcherRegex {
    fn match_rule(&self, url: &Url) -> Vec<&rule::Rule> {
        let mut path = percent_decode(url.path().as_bytes())
            .decode_utf8()
            .unwrap()
            .to_string();

        if url.query().is_some() {
            path = [path, "?".to_string(), url.query().unwrap().to_string()].join("");
        }

        if self.empty.is_some() {
            let empty_match = self.empty.as_ref().unwrap().match_string(&path);

            if empty_match.is_some() {
                return empty_match.unwrap().match_rule(url);
            }
        }

        let mut matched_rules = Vec::new();

        for child in &self.children {
            let child_match = child.match_string(&path);

            if child_match.is_some() {
                matched_rules.append(&mut child_match.unwrap().match_rule(url));
            }
        }

        return matched_rules;
    }

    fn get_rules(&self) -> Vec<&rule::Rule> {
        let mut rules = Vec::new();

        for matcher in &self.children {
            rules.append(&mut matcher.get_rules());
        }

        if self.empty.is_some() {
            rules.append(&mut self.empty.as_ref().unwrap().get_rules());
        }

        return rules;
    }
}
