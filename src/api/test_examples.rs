use std::{collections::HashMap, sync::Arc};

use linked_hash_set::LinkedHashSet;
use serde::{Deserialize, Serialize};

use super::{Example, Rule};
use crate::{
    action::UnitTrace,
    api::{redirection_loop::RedirectionLoop, rules_message::RuleChangeSet},
    router::{Route, Router},
    router_config::RouterConfig,
};

// Input

#[derive(Deserialize, Debug, Clone)]
pub struct TestExamplesInput {
    pub router_config: RouterConfig,
    pub rules: Vec<Rule>,
    pub max_hops: u8,
    #[serde(default)]
    pub project_domains: Vec<String>,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct TestExamplesProjectInput {
    pub change_set: RuleChangeSet,
    pub max_hops: u8,
    #[serde(default)]
    pub project_domains: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub struct TestExamplesOutput {
    pub example_count: u32,
    pub failure_count: u32,
    pub error_count: u32,
    pub first_ten_failures: HashMap<String, FailedRule>,
    pub first_ten_errors: HashMap<String, ErroredRule>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct FailedRule {
    pub rule: Rule,
    pub failed_examples: Vec<FailedExample>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct FailedExample {
    example: Example,
    rule_ids_applied: LinkedHashSet<String>,
    unit_ids_applied: LinkedHashSet<String>,
    unit_ids_not_applied_anymore: LinkedHashSet<String>,
    redirection_loop: Option<RedirectionLoop>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ErroredRule {
    pub rule: Rule,
    pub errored_examples: Vec<ErroredExample>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ErroredExample {
    example: Example,
    error: String,
}

impl TestExamplesOutput {
    pub fn from_project(test_examples_input: TestExamplesProjectInput, existing_router: Arc<Router<Rule>>) -> TestExamplesOutput {
        let test_example_router = if test_examples_input.change_set.is_empty() {
            existing_router
        } else {
            Arc::new(test_examples_input.change_set.update_existing_router(existing_router))
        };

        Self::create_result(
            &test_example_router,
            test_examples_input.max_hops,
            test_examples_input.project_domains,
        )
    }

    pub fn create_result_without_project(test_examples_input: TestExamplesInput) -> TestExamplesOutput {
        let mut router = Router::<Rule>::from_config(test_examples_input.router_config.clone());

        for rule in test_examples_input.rules.iter() {
            router.insert(rule.clone());
        }

        Self::create_result(&router, test_examples_input.max_hops, test_examples_input.project_domains)
    }

    fn create_result(router: &Router<Rule>, max_hops: u8, project_domains: Vec<String>) -> TestExamplesOutput {
        let mut results = TestExamplesOutput::default();

        for (id, route) in router.routes() {
            let examples = &route.handler().examples;

            if examples.is_none() {
                continue;
            }

            for example in examples.as_ref().unwrap().iter() {
                Self::test_example(
                    router,
                    example,
                    &mut results,
                    id.as_str(),
                    route.clone(),
                    max_hops,
                    project_domains.clone(),
                );
            }
        }

        results
    }

    pub fn test_example(
        router: &Router<Rule>,
        example: &Example,
        results: &mut TestExamplesOutput,
        id: &str,
        route: Arc<Route<Rule>>,
        max_hops: u8,
        project_domains: Vec<String>,
    ) {
        if example.unit_ids_applied.is_none() {
            return;
        }

        let unit_trace = match UnitTrace::from_example(router, example) {
            Ok(unit_trace) => unit_trace,
            Err(e) => {
                results.add_errored_example(route.handler(), example.clone(), e.to_string());

                return;
            }
        };

        let unit_ids_not_applied_anymore = unit_trace.diff(example.unit_ids_applied.clone().unwrap());

        // If it should match but not unit are applied anymore
        // If it should match but the rule is not applied
        // If it should not match but the rule is applied
        if example.must_match && (!unit_ids_not_applied_anymore.is_empty() || !unit_trace.rule_ids_contains(id))
            || !example.must_match && unit_trace.rule_ids_contains(id)
        {
            results.add_failed_example(
                route.handler(),
                example.clone(),
                unit_trace.get_rule_ids_applied(),
                unit_trace.get_unit_ids_applied(),
                unit_ids_not_applied_anymore,
                None,
            );
        } else {
            let redirection_loop = RedirectionLoop::from_example(router, max_hops, example, project_domains);

            if redirection_loop.has_error_too_many_hops() || redirection_loop.has_error_loop() {
                results.add_failed_example(
                    route.handler(),
                    example.clone(),
                    unit_trace.get_rule_ids_applied(),
                    unit_trace.get_unit_ids_applied(),
                    unit_ids_not_applied_anymore,
                    Some(redirection_loop),
                );
            }
        }

        results.increment_example_count();
    }

    pub fn add_failed_example(
        &mut self,
        rule: &Rule,
        example: Example,
        rule_ids_applied: LinkedHashSet<String>,
        unit_ids_applied: LinkedHashSet<String>,
        unit_ids_not_applied_anymore: LinkedHashSet<String>,
        redirection_loop: Option<RedirectionLoop>,
    ) {
        self.failure_count += 1;
        if self.first_ten_failures.len() <= 10 {
            let failed_example = FailedExample {
                example,
                rule_ids_applied,
                unit_ids_applied,
                unit_ids_not_applied_anymore,
                redirection_loop,
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
