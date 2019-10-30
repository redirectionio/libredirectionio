use crate::filter::header_action;

#[derive(Debug)]
pub struct HeaderAddAction {
    pub name: String,
    pub value: String,
}

impl header_action::HeaderAction for HeaderAddAction {
    fn filter(&self, mut headers: Vec<header_action::Header>) -> Vec<header_action::Header> {
        headers.push(header_action::Header::new(
            self.name.clone(),
            self.value.clone(),
        ));

        headers
    }
}
