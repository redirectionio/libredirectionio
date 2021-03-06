use super::{Action as RedirectionioAction, FilterBodyAction};
use crate::http::wasm::HeaderMap;
use serde_json::from_str as json_decode;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Action {
    #[wasm_bindgen(skip)]
    pub action: Option<RedirectionioAction>,
}

#[wasm_bindgen]
pub struct BodyFilter {
    #[wasm_bindgen(skip)]
    pub filter: Option<FilterBodyAction>,
}

#[wasm_bindgen]
impl Action {
    #[wasm_bindgen(constructor)]
    pub fn new(action_serialized: String) -> Action {
        let action = match json_decode(action_serialized.as_str()) {
            Err(error) => {
                error!("Unable to deserialize \"{}\" to action: {}", action_serialized, error,);

                None
            }
            Ok(action) => Some(action),
        };

        Action { action }
    }

    pub fn empty() -> Action {
        Action { action: None }
    }

    pub fn get_status_code(&mut self, response_status_code: u16) -> u16 {
        if let Some(action) = self.action.as_mut() {
            return action.get_status_code(response_status_code);
        }

        0
    }

    pub fn filter_headers(&mut self, headers: HeaderMap, response_status_code: u16, add_rule_ids_header: bool) -> HeaderMap {
        if self.action.is_none() {
            return headers;
        }

        let action = self.action.as_mut().unwrap();
        let new_headers = action.filter_headers(headers.headers, response_status_code, add_rule_ids_header);

        HeaderMap { headers: new_headers }
    }

    pub fn create_body_filter(&mut self, response_status_code: u16) -> BodyFilter {
        if self.action.is_none() {
            return BodyFilter { filter: None };
        }

        let action = self.action.as_mut().unwrap();
        let filter = action.create_filter_body(response_status_code);

        BodyFilter { filter }
    }
}

#[wasm_bindgen]
impl BodyFilter {
    pub fn is_null(&self) -> bool {
        self.filter.is_none()
    }

    pub fn filter(&mut self, data: Vec<u8>) -> Vec<u8> {
        if self.filter.is_none() {
            return data;
        }

        let filter = self.filter.as_mut().unwrap();

        let body = match String::from_utf8(data) {
            Err(error) => return error.into_bytes(),
            Ok(body) => body,
        };

        let new_body = filter.filter(body);

        new_body.into_bytes()
    }

    pub fn end(&mut self) -> Vec<u8> {
        if self.filter.is_none() {
            return Vec::new();
        }

        let filter = self.filter.as_mut().unwrap();
        let end = filter.end();

        self.filter = None;

        end.into_bytes()
    }
}
