use http::HeaderMap;
use http::header::HeaderName;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Hash)]
pub struct Header {
    pub name: String,
    pub value: String,
}

impl Header {
    pub fn create_header_map(headers: Vec<Header>) -> HeaderMap<String> {
        let mut header_map = HeaderMap::<String>::default();

        for header in headers {
            let name = match HeaderName::from_bytes(header.name.as_bytes()) {
                Ok(name) => name,
                Err(_) => {
                    log::error!("unable to create header name from: {}", header.name);

                    continue;
                }
            };

            if header_map.contains_key(&name) {
                header_map.append(name, header.value);
            } else {
                header_map.insert(name, header.value);
            }
        }

        header_map
    }
}
