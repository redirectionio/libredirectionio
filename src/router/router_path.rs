use crate::router;
use crate::router::url_matcher::UrlMatcher;
use crate::router::url_matcher_regex::UrlMatcherItem;
use crate::router::url_matcher_regex::UrlMatcherRegex;
use crate::router::url_matcher_rules::UrlMatcherRules;
use std::cmp;
use std::collections::HashMap;
use url::Url;

#[derive(Debug)]
pub struct RouterPath {
    matcher: Box<dyn UrlMatcher>,
    static_rules: HashMap<String, Vec<router::rule::Rule>>,
}

impl router::Router for RouterPath {
    fn match_rule(&self, url: Url) -> Result<Vec<&router::rule::Rule>, Box<dyn std::error::Error>> {
        let mut path = url.path().to_string();

        if url.query().is_some() {
            path = [path, "?".to_string(), url.query().unwrap().to_string()].join("");
        }

        let mut rules = self.matcher.match_rule(&url, path.as_str())?;

        if self.static_rules.contains_key(path.as_str()) {
            rules.extend(self.static_rules.get(path.as_str()).unwrap())
        }

        Ok(rules)
    }

    fn trace(
        &self,
        url: Url,
    ) -> Result<Vec<router::rule::RouterTraceItem>, Box<dyn std::error::Error>> {
        let mut path = url.path().to_string();

        if url.query().is_some() {
            path = [path, "?".to_string(), url.query().unwrap().to_string()].join("");
        }

        let mut traces = self.matcher.trace(&url, path.as_str())?;

        traces.push(router::rule::RouterTraceItem {
            matches: self.static_rules.contains_key(path.as_str()),
            prefix: "".to_string(),
            rules_evaluated: Vec::new(),
            rules_matches: self.static_rules.get(path.as_str()).unwrap().clone(),
        });

        Ok(traces)
    }

    fn build_cache(&mut self, cache_limit: u64, level: u64) -> u64 {
        self.matcher.build_cache(cache_limit, level)
    }
}

impl RouterPath {
    pub fn new(rules: Vec<router::rule::Rule>) -> Result<RouterPath, regex::Error> {
        let mut static_rules: HashMap<String, Vec<router::rule::Rule>> = HashMap::new();
        let mut regex_rules: Vec<router::rule::Rule> = Vec::new();

        for rule in rules {
            if rule.static_path.is_some() {
                let static_path = rule.static_path.as_ref().unwrap().clone();

                if !static_rules.contains_key(static_path.as_str()) {
                    static_rules.insert(static_path.clone(), Vec::new());
                }

                static_rules
                    .get_mut(static_path.as_str())
                    .unwrap()
                    .push(rule);

                continue;
            }

            regex_rules.push(rule)
        }

        let rule_map = create_prefixed_map_rules(regex_rules, "".to_string());
        let matcher_generic = build_matcher_tree("".to_string(), rule_map, 0)?;

        Ok(RouterPath {
            matcher: matcher_generic,
            static_rules,
        })
    }
}

pub fn create_prefixed_map_rules(
    rules: Vec<router::rule::Rule>,
    prefix: String,
) -> HashMap<String, Vec<router::rule::Rule>> {
    let mut new_map = HashMap::new();

    for rule in rules {
        let mut rule_prefix = rule.regex.clone().unwrap();

        if !prefix.is_empty() {
            rule_prefix = strip_characters(rule_prefix.as_str(), prefix.as_str());
        }

        if !new_map.contains_key(rule_prefix.as_str()) {
            new_map.insert(rule_prefix.clone(), Vec::new());
        }

        new_map.get_mut(rule_prefix.as_str()).unwrap().push(rule);
    }

    new_map
}

fn strip_characters(original: &str, prefix: &str) -> String {
    let mut result = String::new();
    let mut original_chars = original.chars();
    let mut prefix_chars = prefix.chars();

    loop {
        let original_next_char = original_chars.next();
        let prefix_next_char = prefix_chars.next();

        if original_next_char.is_none() {
            return result;
        }

        if prefix_next_char.is_none() {
            result.push(original_next_char.unwrap());
            break;
        }

        if original_next_char.unwrap() != prefix_next_char.unwrap() {
            result.push(original_next_char.unwrap());
            break;
        }
    }

    loop {
        let original_next_char = original_chars.next();

        if original_next_char.is_none() {
            return result;
        }

        result.push(original_next_char.unwrap());
    }
}

//
fn build_matcher_tree(
    base_prefix: String,
    mut rule_map: HashMap<String, Vec<router::rule::Rule>>,
    level: u64,
) -> Result<Box<dyn router::url_matcher::UrlMatcher>, regex::Error> {
    if rule_map.is_empty() {
        return Ok(Box::new(UrlMatcherRules::new(Vec::new(), level)));
    }

    if rule_map.len() == 1 {
        return Ok(Box::new(UrlMatcherRules::new(
            rule_map.values().next().unwrap().to_vec(),
            level,
        )));
    }

    let mut children = Vec::new();
    let mut empty = Vec::new();

    while !rule_map.is_empty() {
        let (prefix, mut matched, new_rule_map) = common_prefix(rule_map);

        if prefix.is_empty() {
            empty.append(&mut matched);
        } else {
            let new_base_prefix = [base_prefix.as_str(), prefix.as_str()].join("").to_string();
            let prefixed_map_rules = create_prefixed_map_rules(matched, new_base_prefix.clone());

            children.push(UrlMatcherItem::new(
                ["^", base_prefix.as_str(), prefix.as_str()]
                    .join("")
                    .to_string(),
                build_matcher_tree(new_base_prefix, prefixed_map_rules, level + 1)?,
                level,
            )?);
        }

        rule_map = new_rule_map;
    }

    let mut empty_matcher = None;

    if !empty.is_empty() {
        empty_matcher = Some(UrlMatcherItem::new(
            ["^", base_prefix.as_str(), "$"].join("").to_string(),
            Box::new(UrlMatcherRules::new(empty, level)),
            level,
        )?);
    }

    Ok(Box::new(UrlMatcherRegex::new(children, empty_matcher)))
}

fn common_prefix(
    rule_map: HashMap<String, Vec<router::rule::Rule>>,
) -> (
    String,
    Vec<router::rule::Rule>,
    HashMap<String, Vec<router::rule::Rule>>,
) {
    let mut prefix: Option<String> = None;
    let mut matched = Vec::new();
    let mut not_matched = HashMap::new();
    let mut final_prefix = "".to_string();

    for (regex, rules) in rule_map {
        match prefix {
            None => {
                final_prefix = regex.clone();
                prefix = Some(regex.clone());

                for rule in rules {
                    matched.push(rule);
                }
            }
            Some(prefix_unwrap) => {
                prefix = Some(prefix_unwrap.clone());

                if prefix_unwrap.is_empty() && regex.is_empty() {
                    for rule in rules {
                        matched.push(rule);
                    }
                } else {
                    let new_prefix = longest_prefix(prefix_unwrap.clone(), regex.clone());

                    if new_prefix.is_empty() {
                        not_matched.insert(regex, rules);
                    } else {
                        for rule in rules {
                            matched.push(rule);
                        }

                        final_prefix = new_prefix.clone();
                        prefix = Some(new_prefix.clone());
                    }
                }
            }
        }
    }

    (final_prefix, matched, not_matched)
}

fn longest_prefix(left_prefix: String, right_prefix: String) -> String {
    let mut prefix_length = 0;
    let left_prefix_utf8 = to_char_vector(left_prefix.clone());
    let right_prefix_utf8 = to_char_vector(right_prefix.clone());
    let end = cmp::min(left_prefix_utf8.len(), right_prefix_utf8.len());
    let mut i = 0;

    'main: while i < end && left_prefix_utf8[i] == right_prefix_utf8[i] {
        if '(' == left_prefix_utf8[i] {
            let mut n = 1;
            let mut j = 1 + i;

            while j < end && 0 < n {
                j += 1;

                if left_prefix_utf8[j] != right_prefix_utf8[j] {
                    break 'main;
                }

                if '(' == left_prefix_utf8[j] {
                    n += 1;
                } else if ')' == left_prefix_utf8[j] {
                    n -= 1;
                } else if '\\' == left_prefix_utf8[j]
                    && (j + 1 == end || left_prefix_utf8[j + 1] != right_prefix_utf8[j + 1])
                {
                    break;
                }
            }

            if 0 < n {
                break 'main;
            }

            i = j - 1;
        } else if '\\' == left_prefix_utf8[i]
            && (i + 1 == end || left_prefix_utf8[i + 1] != right_prefix_utf8[i + 1])
        {
            break 'main;
        }

        i += 1;
        prefix_length = i;
    }

    let mut new_prefix = left_prefix_utf8.clone();
    new_prefix.truncate(prefix_length);

    new_prefix.into_iter().collect()
}

fn to_char_vector(str: String) -> Vec<char> {
    let mut chars = str.chars();
    let mut vec_chars = Vec::new();

    loop {
        let character = chars.next();

        if character.is_none() {
            break;
        }

        vec_chars.push(character.unwrap());
    }

    vec_chars
}
