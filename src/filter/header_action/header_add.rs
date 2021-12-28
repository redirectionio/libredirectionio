use crate::action::UnitTrace;
use crate::filter::header_action::HeaderAction;
use crate::http::Header;

#[derive(Debug)]
pub struct HeaderAddAction {
    pub name: String,
    pub value: String,
    // in 3.0 make this mandatory
    pub id: Option<String>,
    // in 3.0 make this mandatory
    pub target_hash: Option<String>,
}

impl HeaderAction for HeaderAddAction {
    fn filter(&self, mut headers: Vec<Header>, mut unit_trace: Option<&mut UnitTrace>) -> Vec<Header> {
        headers.push(Header {
            name: self.name.clone(),
            value: self.value.clone(),
        });

        if let Some(trace) = unit_trace.as_deref_mut() {
            if let Some(id) = &self.id {
                trace.add_value_computed_by_unit(&id, &self.value);
                if let Some(target_hash) = &self.target_hash {
                    trace.add_unit_id_with_target(target_hash, id);
                }
            }
        }

        headers
    }
}
