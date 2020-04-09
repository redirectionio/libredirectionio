use crate::router;
use crate::regex_radix_tree::RegexRadixTree;
use std::collections::HashMap;
use http::Request;
use crate::router::rule::Rule;

#[derive(Debug)]
pub struct RouterPath {
    regex_tree_rule: RegexRadixTree<Rule>,
    static_rules: HashMap<String, Vec<router::rule::Rule>>,
}

impl router::Router for RouterPath {
    fn match_rule(&self, request: &Request<()>) -> Result<Vec<&router::rule::Rule>, Box<dyn std::error::Error>> {
        let mut path = request.uri().path().to_string();

        if request.uri().query().is_some() {
            path = [path, "?".to_string(), request.uri().query().unwrap().to_string()].join("");
        }

        let mut rules = self.regex_tree_rule.find(path.as_str()).unwrap_or(Vec::new());

        match self.static_rules.get(path.as_str()) {
            None => (),
            Some(static_rules) => {
                rules.extend(static_rules)
            }
        }

        Ok(rules)
    }

    fn trace(
        &self,
        request: &Request<()>,
    ) -> Result<Vec<router::rule::RouterTraceItem>, Box<dyn std::error::Error>> {
        let mut path = request.uri().path().to_string();

        if request.uri().query().is_some() {
            path = [path, "?".to_string(), request.uri().query().unwrap().to_string()].join("");
        }

        // @TODO Implement trace on regex_radix_tree
        let mut traces = Vec::new();

        if self.static_rules.contains_key(path.as_str()) {
            let rules_evaluated = self.static_rules.get(path.as_str()).unwrap().clone();
            let mut rules_matched = Vec::new();

            for rule in &rules_evaluated {
                if rule.is_match(path.as_str())? {
                    rules_matched.push(rule.clone())
                }
            }

            traces.push(router::rule::RouterTraceItem {
                matches: true,
                prefix: path,
                rules_evaluated,
                rules_matches: rules_matched,
            });
        }

        Ok(traces)
    }

    fn build_cache(&mut self, cache_limit: u64, level: u64) -> u64 {
        self.regex_tree_rule.cache(cache_limit, level)
    }
}

impl RouterPath {
    pub fn new(rules: Vec<router::rule::Rule>) -> Result<RouterPath, regex::Error> {
        let mut static_rules: HashMap<String, Vec<router::rule::Rule>> = HashMap::new();
        let mut regex_tree_rule: RegexRadixTree<router::rule::Rule> = RegexRadixTree::new();

        for rule in rules {
            match rule.static_path.as_ref() {
                Some(static_path) => {
                    if !static_rules.contains_key(static_path.as_str()) {
                        static_rules.insert(static_path.clone(), Vec::new());
                    }

                    static_rules
                        .get_mut(static_path.as_str())
                        .unwrap()
                        .push(rule);
                },
                None => regex_tree_rule.insert(rule)
            }
        }

        Ok(RouterPath {
            regex_tree_rule,
            static_rules,
        })
    }
}
