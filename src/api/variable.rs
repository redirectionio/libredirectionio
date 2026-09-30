use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    api::Transformer,
    http::{Header, Request},
};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum VariableKind {
    Marker(String),
    RequestHeader {
        name: String,
        default: Option<String>,
    },
    RequestHost,
    RequestMethod,
    RequestPath,
    RequestRemoteAddress,
    RequestScheme,
    RequestTime,
    ResponseHeader {
        name: String,
        default: Option<String>,
    },
    HtmlBody {
        selector: String,
        default: Option<String>,
    },
    #[serde(untagged)]
    Other(serde_json::Value),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Variable {
    pub name: String,
    #[serde(rename = "type")]
    kind: VariableKind,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    transformers: Vec<Transformer>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum VariableValue {
    Value(String),
    HtmlFilter {
        selector: String,
        default: Option<String>,
        transformers: Vec<Transformer>,
    },
    // Resolved once the backend response headers are known, see `VariableValue::resolve_response_headers`.
    ResponseHeader {
        name: String,
        default: Option<String>,
        transformers: Vec<Transformer>,
    },
}

impl Variable {
    pub fn get_value(&self, markers_captured: &HashMap<String, String>, request: &Request) -> VariableValue {
        let mut value = match &self.kind {
            VariableKind::RequestHeader { name, default } => Some(
                request
                    .header_value(name.as_str())
                    .unwrap_or_else(|| default.clone().unwrap_or_default()),
            ),
            VariableKind::RequestHost => request.host.clone(),
            VariableKind::RequestMethod => request.method.clone(),
            VariableKind::RequestPath => Some(request.path_and_query_skipped.original.clone()),
            VariableKind::RequestRemoteAddress => request.remote_addr.map(|addr| addr.to_string()),
            VariableKind::RequestScheme => request.scheme.clone(),
            VariableKind::RequestTime => request.created_at.map(|d| d.to_rfc2822()),
            VariableKind::Marker(marker_name) => markers_captured.get(marker_name.as_str()).cloned(),
            VariableKind::ResponseHeader { name, default } => {
                return VariableValue::ResponseHeader {
                    name: name.clone(),
                    default: default.clone(),
                    transformers: self.transformers.clone(),
                };
            }
            VariableKind::HtmlBody { selector, default } => {
                return VariableValue::HtmlFilter {
                    selector: selector.clone(),
                    default: default.clone(),
                    transformers: self.transformers.clone(),
                };
            }
            VariableKind::Other(_) => None,
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

        VariableValue::Value(value)
    }
}

impl VariableValue {
    pub fn is_response_header(&self) -> bool {
        matches!(self, VariableValue::ResponseHeader { .. })
    }

    pub fn resolve_response_headers(variables: &[(String, VariableValue)], headers: &[Header]) -> Vec<(String, VariableValue)> {
        variables
            .iter()
            .map(|(name, value)| {
                let value = match value {
                    VariableValue::ResponseHeader { name, default, .. } => {
                        let values = headers
                            .iter()
                            .filter(|h| h.name.eq_ignore_ascii_case(name))
                            .map(|h| h.value.as_str())
                            .collect::<Vec<_>>();

                        value.to_static(if values.is_empty() {
                            default.clone().unwrap_or_default()
                        } else {
                            values.join(",")
                        })
                    }
                    _ => value.clone(),
                };

                (name.clone(), value)
            })
            .collect()
    }

    pub fn to_static(&self, mut new_value: String) -> VariableValue {
        match self {
            VariableValue::Value(_) => VariableValue::Value(new_value),
            VariableValue::HtmlFilter { transformers, .. } | VariableValue::ResponseHeader { transformers, .. } => {
                for transformer in transformers {
                    match transformer.to_transform() {
                        None => (),
                        Some(t) => {
                            new_value = t.transform(new_value);
                        }
                    }
                }

                VariableValue::Value(new_value)
            }
        }
    }
}
