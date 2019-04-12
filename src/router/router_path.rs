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
    matcher: Box<UrlMatcher>,
}

impl router::Router for RouterPath {
    fn match_rule(&self, url: Url) -> Vec<&router::rule::Rule> {
        return self.matcher.match_rule(&url);
    }
}

impl RouterPath {
    pub fn new(rules: Vec<router::rule::Rule>, cache: bool) -> RouterPath {
        let rule_map = create_prefixed_map_rules(rules, "".to_string());
        let matcher_generic = build_matcher_tree("".to_string(), rule_map, cache);

        return RouterPath {
            matcher: matcher_generic,
        };
    }

    pub fn get_rules(&self) -> Vec<&router::rule::Rule> {
        return self.matcher.get_rules();
    }
}

pub fn create_prefixed_map_rules(
    rules: Vec<router::rule::Rule>,
    prefix: String,
) -> HashMap<String, Vec<router::rule::Rule>> {
    let mut new_map = HashMap::new();

    for rule in rules {
        let mut rule_prefix = rule.source.regex.clone().unwrap();

        if !prefix.is_empty() {
            rule_prefix = strip_characters(rule_prefix.as_str(), prefix.as_str());
        }

        if !new_map.contains_key(rule_prefix.as_str()) {
            new_map.insert(rule_prefix.clone(), Vec::new());
        }

        new_map.get_mut(rule_prefix.as_str()).unwrap().push(rule);
    }

    return new_map;
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
    cache: bool,
) -> Box<router::url_matcher::UrlMatcher> {
    if rule_map.is_empty() {
        return Box::new(UrlMatcherRules::new(Vec::new()));
    }

    if rule_map.len() == 1 {
        return Box::new(UrlMatcherRules::new(
            rule_map.values().next().unwrap().to_vec(),
        ));
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
                build_matcher_tree(new_base_prefix, prefixed_map_rules, cache),
                cache,
            ));
        }

        rule_map = new_rule_map;
    }

    let mut empty_matcher = None;

    if empty.len() > 0 {
        empty_matcher = Some(UrlMatcherItem::new(
            ["^", base_prefix.as_str(), "$"].join("").to_string(),
            Box::new(UrlMatcherRules::new(empty)),
            cache,
        ));
    }

    return Box::new(UrlMatcherRegex::new(children, empty_matcher));
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

    return (final_prefix, matched, not_matched);
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
                j = j + 1;

                if left_prefix_utf8[j] != right_prefix_utf8[j] {
                    break 'main;
                }

                if '(' == left_prefix_utf8[j] {
                    n = n + 1;
                } else if ')' == left_prefix_utf8[j] {
                    n = n - 1;
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

        i = i + 1;
        prefix_length = i;
    }

    let mut new_prefix = left_prefix_utf8.clone();
    new_prefix.truncate(prefix_length);

    return new_prefix.into_iter().collect();
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

    return vec_chars;
}
