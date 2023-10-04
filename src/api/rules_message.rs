use crate::api::Rule;
use crate::router::Router;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RulesMessage {
    #[serde(rename = "hydra:member")]
    pub rules: Vec<Rule>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RuleChangeSet {
    pub deleted: Vec<String>,
    pub added: Vec<Rule>,
    pub updated: Vec<Rule>,
}

impl RuleChangeSet {
    pub fn update_existing_router(self, existing_router: Arc<Router<Rule>>) -> Router<Rule> {
        let mut new_router = existing_router.as_ref().clone();

        new_router.apply_change_set(self.added, self.updated, self.deleted);

        new_router
    }
}
