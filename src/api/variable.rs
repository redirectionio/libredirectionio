use crate::api::Transformer;
use crate::http::Request;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum VariableKind {
    Marker(String),
    RequestHeader { name: String, default: Option<String> },
    RequestHost,
    RequestMethod,
    RequestPath,
    RequestRemoteAddress,
    RequestScheme,
    RequestTime,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Variable {
    pub name: String,
    #[serde(rename = "type")]
    kind: VariableKind,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    transformers: Vec<Transformer>,
}

impl Variable {
    pub fn get_value(&self, markers_captured: &HashMap<String, String>, request: &Request) -> String {
        let mut value = match &self.kind {
            VariableKind::RequestHeader { name, default } => Some(
                request
                    .header_value(name.as_str())
                    .unwrap_or_else(|| default.clone().unwrap_or_else(|| "".to_string())),
            ),
            VariableKind::RequestHost => request.host.clone(),
            VariableKind::RequestMethod => request.method.clone(),
            VariableKind::RequestPath => Some(request.path_and_query_skipped.original.clone()),
            VariableKind::RequestRemoteAddress => request.remote_addr.map(|addr| addr.to_string()),
            VariableKind::RequestScheme => request.scheme.clone(),
            VariableKind::RequestTime => request.created_at.map(|d| d.to_rfc2822()),
            VariableKind::Marker(marker_name) => markers_captured.get(marker_name.as_str()).cloned(),
        }
        .unwrap_or_default();

        for transformer in &self.transformers {
            match transformer.to_transform() {
                None => (),
                Some(t) => {
                    value = t.transform(value);
                }
            }
        }

        value
    }
}
