use std::os::raw::c_char;
use std::ffi::{CStr, CString};
use std::ptr::null;

pub unsafe fn c_char_to_str(ptr: *const c_char) -> Option<&'static str> {
    if ptr.is_null() {
        return None;
    }

    match CStr::from_ptr(ptr).to_str() {
        Err(error) => {
            error!(
                "Unable to create string for '{}': {}",
                String::from_utf8_lossy(CStr::from_ptr(ptr).to_bytes()),
                error,
            );

            None
        },
        Ok(string) => Some(string),
    }
}

pub unsafe fn string_to_c_char(str: String) -> *const c_char {
    let string = match CString::new(str.as_str()) {
        Err(error) => {
            error!(
                "Cannot create c string {}: {}",
                str,
                error,
            );

            return null();
        },
        Ok(string) => string,
    };

    string.into_raw()
}
