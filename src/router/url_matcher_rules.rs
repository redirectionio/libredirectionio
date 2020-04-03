use crate::router::rule;
use crate::router::url_matcher;
use url::Url;

#[derive(Debug)]
pub struct UrlMatcherRules {
    rules: Vec<rule::Rule>,
    level: u64,
}

impl UrlMatcherRules {
    pub fn new(rules: Vec<rule::Rule>, level: u64) -> UrlMatcherRules {
        UrlMatcherRules { rules, level }
    }
}

impl url_matcher::UrlMatcher for UrlMatcherRules {
    fn match_rule(
        &self,
        _url: &Url,
        path: &str,
    ) -> Result<Vec<&rule::Rule>, Box<dyn std::error::Error>> {
        let mut matched_rules = Vec::new();

        for rule in self.rules.as_slice() {
            if rule.is_match(path)? {
                matched_rules.push(rule);
            }
        }

        Ok(matched_rules)
    }

    fn build_cache(&mut self, cache_limit: u64, level: u64) -> u64 {
        if self.level != level {
            return cache_limit;
        }

        if cache_limit == 0 {
            return cache_limit;
        }

        let mut new_cache_limit = cache_limit;

        for rule in &mut self.rules {
            let result = rule.compile(true);

            if result.is_ok() {
                new_cache_limit -= 1;
            }

            if new_cache_limit == 0 {
                break;
            }
        }

        new_cache_limit
    }

    fn trace(
        &self,
        url: &Url,
        path: &str,
    ) -> Result<Vec<rule::RouterTraceItem>, Box<dyn std::error::Error>> {
        let rules = self.match_rule(url, path)?;
        let mut rules_matched = Vec::new();

        for rule in rules {
            rules_matched.push(rule.clone());
        }

        Ok(vec![rule::RouterTraceItem {
            matches: !rules_matched.is_empty(),
            prefix: "".to_string(),
            rules_evaluated: self.rules.clone(),
            rules_matches: rules_matched,
        }])
    }
}
