use std::collections::HashMap;

use crate::{
    action::{Action, UnitTrace},
    http::Request,
    router::{Router, RouterConfig},
};

use super::{Example, Rule};
use linked_hash_set::LinkedHashSet;
use serde::{Deserialize, Serialize};

// Input

#[derive(Deserialize, Debug, Clone)]
pub struct TestExamplesInput {
    pub router_config: RouterConfig,
    pub rules: TmpRules,
}

// FIXME: find a way to avoid creating this structure.
// It would be more convenient to inline the structure
#[derive(Deserialize, Debug, Clone)]
pub struct TmpRules {
    #[serde(rename = "hydra:member")]
    pub rules: Vec<Rule>,
}
// Output

#[derive(Serialize, Debug, Clone, Default)]
pub struct TestExamplesOutput {
    pub example_count: u32,
    pub failure_count: u32,
    pub error_count: u32,
    pub first_ten_failures: HashMap<String, FailedRule>,
    pub first_ten_errors: HashMap<String, ErroredRule>,
}

#[derive(Serialize, Debug, Clone)]
pub struct FailedRule {
    pub rule: Rule,
    pub failed_examples: Vec<FailedExample>,
}

#[derive(Serialize, Debug, Clone)]
pub struct FailedExample {
    example: Example,
    rule_ids_applied: LinkedHashSet<String>,
    unit_ids_applied: LinkedHashSet<String>,
    unit_ids_not_applied_anymore: LinkedHashSet<String>,
}

#[derive(Serialize, Debug, Clone)]
pub struct ErroredRule {
    pub rule: Rule,
    pub errored_examples: Vec<ErroredExample>,
}

#[derive(Serialize, Debug, Clone)]
pub struct ErroredExample {
    example: Example,
    error: String,
}

// Implementation

impl TestExamplesOutput {
    pub fn create_result(test_examples_input: TestExamplesInput) -> TestExamplesOutput {
        let mut router = Router::<Rule>::from_config(test_examples_input.router_config.clone());

        for rule in test_examples_input.rules.rules.iter() {
            router.insert(rule.clone().into_route(&test_examples_input.router_config));
        }

        router.cache(10000);

        let mut results = TestExamplesOutput::default();

        for rule in test_examples_input.rules.rules.iter() {
            if rule.examples.is_none() {
                continue;
            }

            for example in rule.examples.as_ref().unwrap().iter() {
                if example.unit_ids_applied.is_none() {
                    continue;
                }

                let request = match Request::from_example(&test_examples_input.router_config, example) {
                    Ok(request) => request,
                    Err(e) => {
                        results.add_errored_example(rule, example.clone(), e.to_string());
                        continue;
                    }
                };
                let routes = router.match_request(&request);
                let mut action = Action::from_routes_rule(routes, &request);

                let mut unit_trace = UnitTrace::default();

                let action_status_code = action.get_status_code(0, Some(&mut unit_trace));
                let (final_status_code, backend_status_code) = if action_status_code != 0 {
                    (action_status_code, action_status_code)
                } else {
                    // We call the backend and get a response code
                    let backend_status_code = example.response_status_code.unwrap_or(200);
                    let final_status_code = action.get_status_code(backend_status_code, Some(&mut unit_trace));
                    (final_status_code, backend_status_code)
                };

                action.filter_headers(Vec::new(), backend_status_code, false, Some(&mut unit_trace));

                if let Some(mut body_filter) = action.create_filter_body(backend_status_code, &[]) {
                    let body = "<!DOCTYPE html>
<html>
    <head>
    </head>
    <body>
    </body>
</html>";

                    body_filter.filter(body.into(), Some(&mut unit_trace));
                    body_filter.end(Some(&mut unit_trace));
                }

                action.should_log_request(true, final_status_code, Some(&mut unit_trace));

                unit_trace.squash_with_target_unit_traces();

                let unit_ids_not_applied_anymore = unit_trace.diff(example.unit_ids_applied.clone().unwrap());

                // If it should match but not unit are applied anymore
                // If it should match but the rule is not applied
                // If it should not matche but the rule is applied
                if example.must_match && (!unit_ids_not_applied_anymore.is_empty() || !unit_trace.rule_ids_contains(rule.id.as_str()))
                    || !example.must_match && unit_trace.rule_ids_contains(rule.id.as_str())
                {
                    results.add_failed_example(
                        rule,
                        example.clone(),
                        unit_trace.get_rule_ids_applied(),
                        unit_trace.get_unit_ids_applied(),
                        unit_ids_not_applied_anymore,
                    );
                }

                results.increment_example_count();
            }
        }

        results
    }

    pub fn add_failed_example(
        &mut self,
        rule: &Rule,
        example: Example,
        rule_ids_applied: LinkedHashSet<String>,
        unit_ids_applied: LinkedHashSet<String>,
        unit_ids_not_applied_anymore: LinkedHashSet<String>,
    ) {
        self.failure_count += 1;
        if self.first_ten_failures.len() <= 10 {
            let failed_example = FailedExample {
                example,
                rule_ids_applied,
                unit_ids_applied,
                unit_ids_not_applied_anymore,
            };

            let failed_rule = self
                .first_ten_failures
                .entry(rule.id.clone())
                .or_insert_with(|| FailedRule::new((*rule).clone()));

            failed_rule.failed_examples.push(failed_example);
        }
    }

    pub fn add_errored_example(&mut self, rule: &Rule, example: Example, error: String) {
        self.error_count += 1;
        if self.first_ten_errors.len() <= 10 {
            let errored_example = ErroredExample { example, error };

            let errored_rule = self
                .first_ten_errors
                .entry(rule.id.clone())
                .or_insert_with(|| ErroredRule::new((*rule).clone()));

            errored_rule.errored_examples.push(errored_example);
        }
    }

    pub fn increment_example_count(&mut self) {
        self.example_count += 1;
    }
}

impl FailedRule {
    pub fn new(rule: Rule) -> Self {
        Self {
            rule,
            failed_examples: Vec::new(),
        }
    }
}

impl ErroredRule {
    pub fn new(rule: Rule) -> Self {
        Self {
            rule,
            errored_examples: Vec::new(),
        }
    }
}
