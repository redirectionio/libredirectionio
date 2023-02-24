use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DateTimeConstraint(pub Option<String>, pub Option<String>);
