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
    pub matcher: Box<dyn UrlMatcher>,
    level: u64,
}

#[derive(Debug)]
pub struct UrlMatcherRegex {
    children: Vec<UrlMatcherItem>,
    empty: Option<UrlMatcherItem>,
}

impl UrlMatcherItem {
    pub fn new(
        regex_str: String,
        matcher: Box<dyn UrlMatcher>,
        level: u64,
    ) -> Result<UrlMatcherItem, regex::Error> {
        Ok(UrlMatcherItem {
            regex: regex_str,
            regex_obj: None,
            matcher,
            level,
        })
    }

    fn match_string(&self, value: &str) -> Result<Option<&dyn UrlMatcher>, regex::Error> {
        if self.regex_obj.is_some() {
            let regex = self.regex_obj.as_ref().unwrap();

            if regex.is_match(value) {
                return Ok(Some(self.matcher.as_ref()));
            }

            return Ok(None);
        }

        let regex = Regex::new(self.regex.as_str())?;

        if regex.is_match(value) {
            return Ok(Some(self.matcher.as_ref()));
        }

        Ok(None)
    }

    fn build_cache(&mut self, cache_limit: u64, level: u64) -> u64 {
        if cache_limit == 0 {
            return cache_limit;
        }

        if level < self.level {
            return cache_limit;
        }

        if level != self.level {
            return self.matcher.build_cache(cache_limit, level);
        }

        if self.regex_obj.is_some() {
            return cache_limit;
        }

        let regex = Regex::new(self.regex.as_str());

        if regex.is_err() {
            return cache_limit;
        }

        self.regex_obj = Some(regex.unwrap());

        cache_limit - 1
    }

    fn get_rules(&self) -> Vec<&rule::Rule> {
        self.matcher.get_rules()
    }
}

impl UrlMatcherRegex {
    pub fn new(children: Vec<UrlMatcherItem>, empty: Option<UrlMatcherItem>) -> UrlMatcherRegex {
        UrlMatcherRegex { children, empty }
    }
}

impl url_matcher::UrlMatcher for UrlMatcherRegex {
    fn match_rule(
        &self,
        url: &Url,
        path: &str,
    ) -> Result<Vec<&rule::Rule>, Box<dyn std::error::Error>> {
        if self.empty.is_some() {
            let empty_match = self.empty.as_ref().unwrap().match_string(&path)?;

            if let Some(empty_matcher) = empty_match {
                return empty_matcher.match_rule(url, path);
            }
        }

        let mut matched_rules = Vec::new();

        for child in &self.children {
            let child_match = child.match_string(path)?;

            if let Some(child_matcher) = child_match {
                matched_rules.append(&mut child_matcher.match_rule(url, path)?);
            }
        }

        Ok(matched_rules)
    }

    fn build_cache(&mut self, cache_limit: u64, level: u64) -> u64 {
        let mut new_cache_limit = cache_limit;

        if self.empty.is_some() {
            new_cache_limit = self
                .empty
                .as_mut()
                .unwrap()
                .build_cache(new_cache_limit, level);
        }

        for matcher in &mut self.children {
            new_cache_limit = matcher.build_cache(new_cache_limit, level);
        }

        new_cache_limit
    }

    fn trace(
        &self,
        url: &Url,
        path: &str,
    ) -> Result<Vec<rule::RouterTraceItem>, Box<dyn std::error::Error>> {
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

            match child_match {
                Some(matcher) => {
                    traces.push(rule::RouterTraceItem {
                        matches: true,
                        prefix: child.regex.clone(),
                        rules_evaluated: Vec::new(),
                        rules_matches: Vec::new(),
                    });

                    traces.append(matcher.trace(url, path)?.as_mut());
                },
                None => {
                    traces.push(rule::RouterTraceItem {
                        matches: false,
                        prefix: child.regex.clone(),
                        rules_evaluated: Vec::new(),
                        rules_matches: Vec::new(),
                    });
                },
            };
        }

        Ok(traces)
    }

    fn get_rules(&self) -> Vec<&rule::Rule> {
        let mut rules = Vec::new();

        for matcher in &self.children {
            rules.append(&mut matcher.get_rules());
        }

        if self.empty.is_some() {
            rules.append(&mut self.empty.as_ref().unwrap().get_rules());
        }

        rules
    }
}
