use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BodyFilter {
    action: String,
    value: String,
    element_tree: Vec<String>,
    css_selector: Option<String>,
}
