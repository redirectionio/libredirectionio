use crate::router::rule;
use crate::router::url_matcher;
use url::percent_encoding::percent_decode;
use url::Url;

#[derive(Debug)]
pub struct UrlMatcherRules {
    rules: Vec<rule::Rule>,
}

impl UrlMatcherRules {
    pub fn new(rules: Vec<rule::Rule>) -> UrlMatcherRules {
        UrlMatcherRules { rules }
    }
}

impl url_matcher::UrlMatcher for UrlMatcherRules {
    fn match_rule(&self, url: &Url) -> Vec<&rule::Rule> {
        let mut matched_rules = Vec::new();
        let mut path = percent_decode(url.path().as_bytes())
            .decode_utf8()
            .unwrap()
            .to_string();

        if url.query() != None {
            path = [path, "?".to_string(), url.query().unwrap().to_string()].join("");
        }

        for rule in self.rules.as_slice() {
            if rule.source.is_match(path.as_str()) {
                matched_rules.push(rule);
            }
        }

        return matched_rules;
    }

    fn get_rules(&self) -> Vec<&rule::Rule> {
        let mut rules = Vec::new();

        for rule in &self.rules {
            rules.push(rule);
        }

        return rules;
    }
}
