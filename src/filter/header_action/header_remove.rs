use crate::filter::header_action;

#[derive(Debug)]
pub struct HeaderRemoveAction {
    pub name: String,
}

impl header_action::HeaderAction for HeaderRemoveAction {
    fn filter(&self, headers: Vec<header_action::Header>) -> Vec<header_action::Header> {
        let mut new_headers = Vec::new();

        for header in headers {
            if header.name != self.name {
                new_headers.push(header);
            }
        }

        new_headers
    }
}
