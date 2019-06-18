extern crate regex;

use crate::router::rule;
use crate::router::url_matcher;
use crate::router::url_matcher::UrlMatcher;
use regex::Regex;
use url::Url;

#[derive(Debug)]
pub struct UrlMatcherItem {
    pub regex: String,
    regex_obj: Option<Regex>,
    pub matcher: Box<UrlMatcher>,
}

#[derive(Debug)]
pub struct UrlMatcherRegex {
    children: Vec<UrlMatcherItem>,
    empty: Option<UrlMatcherItem>,
}

impl UrlMatcherItem {
    pub fn new(
        regex_str: String,
        matcher: Box<UrlMatcher>,
        cache: bool,
    ) -> Result<UrlMatcherItem, regex::Error> {
        let mut regex_obj = None;

        if cache {
            regex_obj = Some(Regex::new(regex_str.as_str())?);
        }

        return Ok(UrlMatcherItem {
            regex: regex_str,
            regex_obj,
            matcher,
        });
    }

    fn match_string(&self, value: &str) -> Result<Option<&Box<UrlMatcher>>, regex::Error> {
        if self.regex_obj.is_some() {
            let regex = self.regex_obj.as_ref().unwrap();

            if regex.is_match(value) {
                return Ok(Some(&self.matcher));
            }

            return Ok(None);
        }

        let regex = Regex::new(self.regex.as_str())?;

        if regex.is_match(value) {
            return Ok(Some(&self.matcher));
        }

        return Ok(None);
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
    fn match_rule(&self, url: &Url, path: &str) -> Result<Vec<&rule::Rule>, Box<dyn std::error::Error>> {
        if self.empty.is_some() {
            let empty_match = self.empty.as_ref().unwrap().match_string(&path)?;

            if empty_match.is_some() {
                return empty_match.unwrap().match_rule(url, path);
            }
        }

        let mut matched_rules = Vec::new();

        for child in &self.children {
            let child_match = child.match_string(path)?;

            if child_match.is_some() {
                matched_rules.append(&mut child_match.unwrap().match_rule(url, path)?);
            }
        }

        return Ok(matched_rules);
    }

    fn trace(&self, url: &Url, path: &str) -> Result<Vec<rule::RouterTraceItem>, Box<dyn std::error::Error>> {
        let mut traces = Vec::new();

        if self.empty.is_some() {
            let empty_match = self.empty.as_ref().unwrap().match_string(&path)?;

            if empty_match.is_some() {
                traces.push(rule::RouterTraceItem {
                    matches: true,
                    prefix: self.empty.as_ref().unwrap().regex.clone(),
                    rules_evaluated: Vec::new(),
                    rules_matches: Vec::new(),
                });

                traces.append(empty_match.as_ref().unwrap().trace(url, path)?.as_mut());

                return Ok(traces);
            }
        }

        for child in &self.children {
            let child_match = child.match_string(path)?;

            if child_match.is_some() {
                traces.push(rule::RouterTraceItem {
                    matches: true,
                    prefix: child.regex.clone(),
                    rules_evaluated: Vec::new(),
                    rules_matches: Vec::new(),
                });

                traces.append(child_match.unwrap().trace(url, path)?.as_mut());
            } else {
                traces.push(rule::RouterTraceItem {
                    matches: false,
                    prefix: child.regex.clone(),
                    rules_evaluated: Vec::new(),
                    rules_matches: Vec::new(),
                });
            }
        }

        return Ok(traces);
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
