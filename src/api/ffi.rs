use crate::ffi_helpers::c_char_to_str;
use crate::api::RulesMessage;
use std::os::raw::c_char;
use serde_json::from_str as json_decoded;
use wasm_bindgen::__rt::core::ptr::null;

#[no_mangle]
pub unsafe extern fn redirectionio_api_create_rules_message_from_json(content: *mut c_char) -> *const RulesMessage {
    let message_string = match c_char_to_str(content) {
        None => return null() as *const RulesMessage,
        Some(str) => str,
    };

    match json_decoded(message_string) {
        Err(error) => {
            error!("{}", error);
            null() as *const RulesMessage
        },
        Ok(message) => {
            Box::into_raw(Box::new(message))
        }
    }
}
