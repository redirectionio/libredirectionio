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

    pub fn get_status_code(&self, response_status_code: u16) -> u16 {
        if let Some(action) = self.action.as_ref() {
            return action.get_status_code(response_status_code);
        }

        0
    }

    pub fn filter_headers(&self, headers: HeaderMap, response_status_code: u16, add_rule_ids_header: bool) -> HeaderMap {
        if self.action.is_none() {
            return headers;
        }

        let action = self.action.as_ref().unwrap();
        let new_headers = action.filter_headers(headers.headers, response_status_code, add_rule_ids_header);

        HeaderMap { headers: new_headers }
    }

    pub fn create_body_filter(&self, response_status_code: u16) -> BodyFilter {
        if self.action.is_none() {
            return BodyFilter { filter: None };
        }

        let action = self.action.as_ref().unwrap();
        let filter = action.create_filter_body(response_status_code);

        BodyFilter { filter }
    }
}

#[wasm_bindgen]
impl BodyFilter {
    pub fn is_null(&self) -> bool {
        self.filter.is_none()
    }

    pub fn filter(&mut self, body: String) -> String {
        if self.filter.is_none() {
            return body;
        }

        let filter = self.filter.as_mut().unwrap();

        filter.filter(body)
    }

    pub fn end(&mut self) -> String {
        if self.filter.is_none() {
            return "".to_string();
        }

        let filter = self.filter.as_mut().unwrap();
        let end = filter.end();

        self.filter = None;

        end
    }
}
