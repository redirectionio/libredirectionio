use crate::{action::UnitTrace, filter::header_action::HeaderAction, http::Header};

#[derive(Debug)]
pub struct HeaderReplaceAction {
    pub name: String,
    pub value: String,
    // in 3.0 make this mandatory
    pub id: Option<String>,
    // in 3.0 make this mandatory
    pub target_hash: Option<String>,
}

// Replace (but do not add if not found) a header
impl HeaderAction for HeaderReplaceAction {
    fn filter(&self, headers: Vec<Header>, mut unit_trace: Option<&mut UnitTrace>) -> Vec<Header> {
        let mut new_headers = Vec::new();

        for header in headers {
            if header.name.to_lowercase() == self.name.to_lowercase() {
                new_headers.push(Header {
                    name: self.name.clone(),
                    value: self.value.clone(),
                });

                if let (Some(trace), Some(id)) = (unit_trace.as_deref_mut(), &self.id) {
                    trace.add_value_computed_by_unit(id, &self.value);

                    if let Some(target_hash) = &self.target_hash {
                        trace.override_unit_id_with_target(target_hash, id);
                    }
                }
            } else {
                new_headers.push(header);
            }
        }

        new_headers
    }
}
