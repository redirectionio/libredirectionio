use crate::filter::header_action::HeaderAction;
use crate::http::Header;

#[derive(Debug)]
pub struct HeaderOverrideAction {
    pub name: String,
    pub value: String,
}

impl HeaderAction for HeaderOverrideAction {
    fn filter(&self, headers: Vec<Header>) -> Vec<Header> {
        let mut new_headers = Vec::new();
        let mut found = false;

        for header in headers {
            if header.name.to_lowercase() != self.name.to_lowercase() {
                new_headers.push(header);
            } else {
                found = true;
                new_headers.push(Header {
                    name: self.name.clone(),
                    value: self.value.clone(),
                });
            }
        }

        if !found {
            new_headers.push(Header {
                name: self.name.clone(),
                value: self.value.clone(),
            });
        }

        new_headers
    }
}
