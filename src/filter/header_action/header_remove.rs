use crate::filter::header_action::HeaderAction;
use crate::http::Header;

#[derive(Debug)]
pub struct HeaderRemoveAction {
    pub name: String,
}

impl HeaderAction for HeaderRemoveAction {
    fn filter(&self, headers: Vec<Header>) -> Vec<Header> {
        let mut new_headers = Vec::new();

        for header in headers {
            if header.name.to_lowercase() != self.name.to_lowercase() {
                new_headers.push(header);
            }
        }

        new_headers
    }
}
