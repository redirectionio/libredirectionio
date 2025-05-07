use std::{collections::HashSet, sync::Arc};

use serde::{Deserialize, Serialize};

use crate::{api::Rule, router::Router};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
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
