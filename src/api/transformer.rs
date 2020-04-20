use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::router::{Transformer as RouteTransformer, Transform, Camelize, Uppercase, Underscorize, Slice, Replace, Dasherize, Lowercase};
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transformer {
    #[serde(rename = "type")]
    pub kind: Option<String>,
    pub options: Option<HashMap<String, String>>,
}

impl Transformer {
    pub fn to_transform(&self) -> Option<Box<dyn Transform>> {
        match self.kind.as_ref() {
            None => None,
            Some(kind) => match kind.as_str() {
                "camelize" => Some(Box::new(Camelize::new())),
                "dasherize" => Some(Box::new(Dasherize::new())),
                "lowercase" => Some(Box::new(Lowercase::new())),
                "replace" => {
                    match self.options.as_ref() {
                        None => None,
                        Some(options) => {
                            if !options.contains_key("something") || !options.contains_key("with") {
                                return None;
                            }

                            Some(Box::new(Replace::new(
                                options.get("something").unwrap().clone(),
                                options.get("with").unwrap().clone(),
                            )))
                        }
                    }
                },
                "slice" => {
                    match self.options.as_ref() {
                        None => None,
                        Some(options) => {
                            if !options.contains_key("from") || !options.contains_key("to") {
                                return None;
                            }

                            let from = usize::from_str(options.get("from").unwrap()).unwrap_or(0);
                            let to = match usize::from_str(options.get("to").unwrap()) {
                                Err(_) => None,
                                Ok(value) => Some(value),
                            };

                            Some(Box::new(Slice::new(
                                from,
                                to,
                            )))
                        }
                    }
                },
                "underscorize" => Some(Box::new(Underscorize::new())),
                "uppercase" => Some(Box::new(Uppercase::new())),
                _ => None,
            },
        }
    }
}
