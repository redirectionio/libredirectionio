use crate::{action::UnitTrace, filter::header_action::HeaderAction, http::Header};

#[derive(Debug)]
pub struct HeaderDefaultAction {
    pub name: String,
    pub value: String,
    // in 3.0 make this mandatory
    pub id: Option<String>,
    // in 3.0 make this mandatory
    pub target_hash: Option<String>,
}

// Set a header if not present
impl HeaderAction for HeaderDefaultAction {
    fn filter(&self, mut headers: Vec<Header>, unit_trace: Option<&mut UnitTrace>) -> Vec<Header> {
        let mut found = false;

        for header in &headers {
            if header.name.to_lowercase() == self.name.to_lowercase() {
                found = true;
                break;
            }
        }

        if !found {
            headers.push(Header {
                name: self.name.clone(),
                value: self.value.clone(),
            });

            if let (Some(trace), Some(id)) = (unit_trace, &self.id) {
                trace.add_value_computed_by_unit(id, &self.value);

                if let Some(target_hash) = &self.target_hash {
                    trace.add_unit_id_with_target(target_hash, id);
                }
            }
        }

        headers
    }
}
