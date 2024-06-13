use crate::api::Rule;
use crate::router::Router;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RulesMessage {
    #[serde(rename = "hydra:member")]
    pub rules: Vec<Rule>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RuleChangeSet {
    pub added: Vec<Rule>,
    pub updated: Vec<Rule>,
    pub deleted: HashSet<String>,
}

impl RuleChangeSet {
    pub fn update_existing_router(self, existing_router: Arc<Router<Rule>>) -> Router<Rule> {
        let mut new_router = existing_router.as_ref().clone();

        new_router.apply_change_set(self.added, self.updated, self.deleted);

        new_router
    }

    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.updated.is_empty() && self.deleted.is_empty()
    }
}
