use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BodyFilter {
    pub action: String,
    pub value: String,
    pub element_tree: Vec<String>,
    pub css_selector: Option<String>,
}
