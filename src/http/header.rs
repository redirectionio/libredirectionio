use http::header::HeaderName;
use http::HeaderMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Header {
    pub name: String,
    pub value: String,
}

impl Header {
    pub fn create_header_map(headers: Vec<Header>) -> HeaderMap<String> {
        let mut header_map = HeaderMap::<String>::default();

        for header in headers {
            let name = HeaderName::from_bytes(header.name.as_bytes()).unwrap();

            if header_map.contains_key(&name) {
                header_map.append(name, header.value);
            } else {
                header_map.insert(name, header.value);
            }
        }

        header_map
    }
}
