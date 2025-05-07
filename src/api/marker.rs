use serde::{Deserialize, Serialize};

use crate::{api::Transformer, marker::Transform};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Marker {
    pub name: String,
    pub regex: String,
    #[serde(default)]
    pub transformers: Vec<Transformer>,
}

impl Transform for Marker {
    fn transform(&self, mut value: String) -> String {
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
