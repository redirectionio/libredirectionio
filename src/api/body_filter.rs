use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HTMLBodyFilter {
    pub action: String,
    pub value: String,
    pub element_tree: Vec<String>,
    pub css_selector: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum BodyFilter {
    HTML(HTMLBodyFilter),
}
