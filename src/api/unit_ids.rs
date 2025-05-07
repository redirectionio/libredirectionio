use std::{collections::HashMap, sync::Arc};

use serde::{Deserialize, Serialize};

use crate::{
    action::UnitTrace,
    api::{Example, Rule, rules_message::RuleChangeSet},
    router::Router,
    router_config::RouterConfig,
};

// Input

#[derive(Deserialize, Debug, Clone)]
pub struct UnitIdsInput {
    pub router_config: RouterConfig,
    pub rules: Vec<Rule>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct UnitIdsProjectInput {
    pub change_set: RuleChangeSet,
}

// Output

#[derive(Serialize, Debug, Clone, Default)]
pub struct UnitIdsOutput {
    pub rules: HashMap<String, RuleOutput>,
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct RuleOutput {
    pub examples: Vec<Example>,
}

// Implementation

impl UnitIdsOutput {
    #[cfg(feature = "router")]
    pub fn create_result_from_project(unit_ids_input: UnitIdsProjectInput, existing_router: Arc<Router<Rule>>) -> UnitIdsOutput {
        let unit_ids_router = unit_ids_input.change_set.update_existing_router(existing_router);

        Self::create_result(&unit_ids_router)
    }

    #[cfg(feature = "router")]
    pub fn create_result_without_project(unit_ids_input: UnitIdsInput) -> UnitIdsOutput {
        let mut router = Router::<Rule>::from_config(unit_ids_input.router_config.clone());

        for rule in unit_ids_input.rules.iter() {
            router.insert(rule.clone());
        }

        router.cache(None);

        Self::create_result(&router)
    }

    #[cfg(feature = "router")]
    fn create_result(router: &Router<Rule>) -> UnitIdsOutput {
        let mut rules = HashMap::new();

        for (id, route) in router.routes() {
            let examples = &route.handler().examples;

            if examples.is_none() {
                continue;
            }

            let mut examples_output = Vec::new();

            for example in examples.as_ref().unwrap() {
                let unit_trace = match UnitTrace::from_example(router, example) {
                    Ok(unit_trace) => unit_trace,
                    Err(_) => {
                        examples_output.push(example.clone());
                        continue;
                    }
                };

                let mut final_example = example.clone();
                final_example.unit_ids_applied = Some(unit_trace.get_unit_ids_applied().into_iter().collect());
                examples_output.push(final_example);
            }

            rules.insert(id.clone(), RuleOutput { examples: examples_output });
        }

        UnitIdsOutput { rules }
    }
}
