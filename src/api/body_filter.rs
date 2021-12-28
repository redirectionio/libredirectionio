use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HTMLBodyFilter {
    pub action: String,
    pub value: String,
    pub inner_value: Option<String>,
    pub element_tree: Vec<String>,
    pub css_selector: Option<String>,
    pub id: Option<String>,
    pub target_hash: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TextBodyFilter {
    pub action: TextAction,
    pub content: String,
    pub id: Option<String>,
    pub target_hash: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TextAction {
    #[serde(rename = "append_text")]
    Append,
    #[serde(rename = "prepend_text")]
    Prepend,
    #[serde(rename = "replace_text")]
    Replace,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum BodyFilter {
    Text(TextBodyFilter),
    HTML(HTMLBodyFilter),
}
