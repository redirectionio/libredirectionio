use std::collections::HashMap;

use linked_hash_set::LinkedHashSet;
use serde::{Deserialize, Serialize};

use crate::{
    action::run::ExampleRun,
    api::{Example, Rule},
    router::Router,
};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct WithTargetUnitTrace {
    unit_ids_applied_by_key: HashMap<String, LinkedHashSet<String>>,
}

impl WithTargetUnitTrace {
    fn add_unit_id(&mut self, target: &str, unit_id: &str) {
        let unit_ids = self.unit_ids_applied_by_key.entry(target.to_string()).or_default();
        unit_ids.insert(unit_id.to_string());
    }

    fn override_unit_id(&mut self, target: &str, unit_id: &str) {
        self.unit_ids_applied_by_key.remove_entry(target);
        self.add_unit_id(target, unit_id);
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct UnitTrace {
    pub(crate) rule_ids_applied: LinkedHashSet<String>,
    unit_ids_applied: LinkedHashSet<String>,
    unit_ids_seen: LinkedHashSet<String>,
    value_computed_by_units: HashMap<String, String>,
    #[serde(skip_serializing)]
    with_target_unit_trace: WithTargetUnitTrace,
}

impl UnitTrace {
    #[cfg(feature = "router")]
    pub fn from_example(router: &Router<Rule>, example: &Example) -> Result<Self, http::Error> {
        let run = ExampleRun::new(router, example)?;

        Ok(run.unit_trace)
    }

    pub fn add_unit_id(&mut self, unit_id: String) {
        self.unit_ids_applied.insert(unit_id.clone());
        self.unit_ids_seen.insert(unit_id);
    }

    pub fn add_unit_id_with_target(&mut self, target: &str, unit_id: &str) {
        self.with_target_unit_trace.add_unit_id(target, unit_id);
        // Here we don't care about squashing value, since we want all value.
        self.unit_ids_seen.insert(unit_id.to_string());
    }

    pub fn override_unit_id_with_target(&mut self, target: &str, unit_id: &str) {
        self.with_target_unit_trace.override_unit_id(target, unit_id);
        // Here we don't care about squashing value, since we want all value.
        self.unit_ids_seen.insert(unit_id.to_string());
    }

    pub fn squash_with_target_unit_traces(&mut self) {
        let unit_ids = std::mem::take(&mut self.with_target_unit_trace);

        for (_, unit_ids) in unit_ids.unit_ids_applied_by_key {
            for unit_id in unit_ids {
                self.add_unit_id(unit_id)
            }
        }

        // Sort, for stability in tests
        let mut tmp = Vec::from_iter(self.unit_ids_applied.clone());
        tmp.sort();
        self.unit_ids_applied = LinkedHashSet::from_iter(tmp);

        self.with_target_unit_trace = WithTargetUnitTrace::default();
    }

    pub fn add_value_computed_by_unit(&mut self, key: &str, value: &str) {
        self.value_computed_by_units.insert(key.to_string(), value.to_string());
    }

    pub fn diff(&self, other: Vec<String>) -> LinkedHashSet<String> {
        let mut diff = LinkedHashSet::new();

        for unit_id in other {
            if !&self.unit_ids_applied.contains(&unit_id) {
                diff.insert(unit_id);
            }
        }

        diff
    }

    pub fn get_rule_ids_applied(&self) -> LinkedHashSet<String> {
        self.rule_ids_applied.clone()
    }

    pub fn rule_ids_contains(&self, rule_id: &str) -> bool {
        self.rule_ids_applied.contains(rule_id)
    }

    pub fn get_unit_ids_applied(&self) -> LinkedHashSet<String> {
        self.unit_ids_applied.clone()
    }
}
