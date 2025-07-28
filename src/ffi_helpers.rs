use std::{
    ffi::{CStr, CString},
    os::raw::c_char,
    ptr::null,
};

pub fn c_char_to_str(ptr: *const c_char) -> Option<&'static str> {
    if ptr.is_null() {
        return None;
    }

    // SAFETY: ptr is a valid pointer to a C string
    let cstr = unsafe { CStr::from_ptr(ptr) };

    match cstr.to_str() {
        Err(error) => {
            log::error!(
                "unable to create string for '{}': {}",
                String::from_utf8_lossy(cstr.to_bytes()),
                error,
            );

            None
        }
        Ok(string) => Some(string),
    }
}

pub fn string_to_c_char(str: String) -> *const c_char {
    let string = match CString::new(str.as_str()) {
        Err(error) => {
            log::error!("cannot create c string {str}: {error}");

            return null();
        }
        Ok(string) => string,
    };

    string.into_raw()
}
