use crate::filter::header_action;

#[derive(Debug)]
pub struct HeaderReplaceAction {
    pub name: String,
    pub value: String,
}

impl header_action::HeaderAction for HeaderReplaceAction {
    fn filter(&self, headers: Vec<header_action::Header>) -> Vec<header_action::Header> {
        let mut new_headers = Vec::new();

        for header in headers {
            if header.name != self.name {
                new_headers.push(header);
            } else {
                new_headers.push(header_action::Header::new(
                    self.name.clone(),
                    self.value.clone(),
                ));
            }
        }

        new_headers
    }
}
