use serde::{Deserialize, Serialize};

use crate::{api::VariableValue, marker::StaticOrDynamic};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HTMLBodyFilterInnerLegacy {
    pub value: String,
    pub inner_value: Option<String>,
    pub element_tree: Vec<String>,
    pub css_selector: Option<String>,
    pub id: Option<String>,
    pub target_hash: Option<String>,
}

impl HTMLBodyFilterInnerLegacy {
    pub fn clone_with_variables_replaced(&self, variables: &[(String, VariableValue)]) -> HTMLBodyFilterInnerLegacy {
        HTMLBodyFilterInnerLegacy {
            css_selector: self.css_selector.clone(),
            element_tree: self.element_tree.clone(),
            value: StaticOrDynamic::replace(self.value.clone(), variables, false),
            inner_value: Some(StaticOrDynamic::replace(
                self.inner_value.clone().unwrap_or_else(|| self.value.clone()),
                variables,
                false,
            )),
            id: self.id.clone(),
            target_hash: self.target_hash.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HTMLBodyFilterInner {
    pub value: String,
    pub inner_value: Option<String>,
    pub css_selector: String,
    pub id: Option<String>,
    pub target_hash: Option<String>,
}

impl HTMLBodyFilterInner {
    pub fn clone_with_variables_replaced(&self, variables: &[(String, VariableValue)]) -> HTMLBodyFilterInner {
        HTMLBodyFilterInner {
            css_selector: self.css_selector.clone(),
            value: StaticOrDynamic::replace(self.value.clone(), variables, false),
            inner_value: Some(StaticOrDynamic::replace(
                self.inner_value.clone().unwrap_or_else(|| self.value.clone()),
                variables,
                false,
            )),
            id: self.id.clone(),
            target_hash: self.target_hash.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HTMLBodyFilterAppend {
    pub value: String,
    pub inner_value: Option<String>,
    pub css_selector: String,
    pub ignore_css_selector: Option<String>,
    pub id: Option<String>,
    pub target_hash: Option<String>,
}

impl HTMLBodyFilterAppend {
    pub fn clone_with_variables_replaced(&self, variables: &[(String, VariableValue)]) -> HTMLBodyFilterAppend {
        HTMLBodyFilterAppend {
            value: StaticOrDynamic::replace(self.value.clone(), variables, false),
            inner_value: Some(StaticOrDynamic::replace(
                self.inner_value.clone().unwrap_or_else(|| self.value.clone()),
                variables,
                false,
            )),
            css_selector: self.css_selector.clone(),
            ignore_css_selector: self.ignore_css_selector.clone(),
            id: self.id.clone(),
            target_hash: self.target_hash.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HTMLBodyFilterRemove {
    pub css_selector: String,
    pub id: Option<String>,
    pub target_hash: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "action")]
pub enum HTMLBodyFilter {
    #[serde(rename = "append_html")]
    Append(HTMLBodyFilterAppend),
    #[serde(rename = "prepend_html")]
    Prepend(HTMLBodyFilterInner),
    #[serde(rename = "replace_html")]
    Replace(HTMLBodyFilterInner),
    #[serde(rename = "remove_html")]
    Remove(HTMLBodyFilterRemove),
    #[serde(rename = "after_html")]
    After(HTMLBodyFilterInner),
    #[serde(rename = "before_html")]
    Before(HTMLBodyFilterInner),
    #[serde(rename = "append_child")]
    AppendLegacy(HTMLBodyFilterInnerLegacy),
    #[serde(rename = "prepend_child")]
    PrependLegacy(HTMLBodyFilterInnerLegacy),
    #[serde(rename = "replace")]
    ReplaceLegacy(HTMLBodyFilterInnerLegacy),
}

impl HTMLBodyFilter {
    pub fn clone_with_variables_replaced(&self, variables: &[(String, VariableValue)]) -> HTMLBodyFilter {
        match self {
            HTMLBodyFilter::AppendLegacy(inner) => HTMLBodyFilter::AppendLegacy(inner.clone_with_variables_replaced(variables)),
            HTMLBodyFilter::PrependLegacy(inner) => HTMLBodyFilter::PrependLegacy(inner.clone_with_variables_replaced(variables)),
            HTMLBodyFilter::ReplaceLegacy(inner) => HTMLBodyFilter::ReplaceLegacy(inner.clone_with_variables_replaced(variables)),
            HTMLBodyFilter::Append(inner) => HTMLBodyFilter::Append(inner.clone_with_variables_replaced(variables)),
            HTMLBodyFilter::Prepend(inner) => HTMLBodyFilter::Prepend(inner.clone_with_variables_replaced(variables)),
            HTMLBodyFilter::Replace(inner) => HTMLBodyFilter::Replace(inner.clone_with_variables_replaced(variables)),
            HTMLBodyFilter::Remove(inner) => HTMLBodyFilter::Remove(inner.clone()),
            HTMLBodyFilter::After(inner) => HTMLBodyFilter::After(inner.clone_with_variables_replaced(variables)),
            HTMLBodyFilter::Before(inner) => HTMLBodyFilter::Before(inner.clone_with_variables_replaced(variables)),
        }
    }
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
    #[serde(untagged)]
    Other(serde_json::Value),
}
