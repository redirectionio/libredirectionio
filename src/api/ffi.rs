use crate::ffi_helpers::c_char_to_str;
use crate::api::RulesMessage;
use std::os::raw::c_char;
use serde_json::from_str;

#[no_mangle]
pub unsafe extern fn redirectionio_api_create_rules_message_from_json(content: *mut c_char) -> Option<*mut RulesMessage> {
    let message_string = c_char_to_str(content)?;
    let message_result = from_str(message_string);

    if message_result.is_err() {
        error!("{}", message_result.err().unwrap());

        return None;
    }

    let message: RulesMessage = message_result.unwrap();

    Some(Box::into_raw(Box::new(message)))
}

// @TODO Add serialization / deserialization of rules message in bincode
