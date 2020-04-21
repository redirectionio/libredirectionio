use std::os::raw::c_char;
use std::ffi::{CStr, CString};
use std::ffi::OsString;
use std::ptr::null;

pub unsafe fn c_char_to_string(ptr: *mut c_char) -> Option<String> {
    if ptr.is_null() {
        return None;
    }

    let cstring = CString::from_raw(ptr);
    let result = cstring.into_string();

    if result.is_err() {
        error!(
            "Unable to create string: {}",
            result.err().unwrap()
        );

        return None;
    }

    Some(result.unwrap())
}

pub unsafe fn c_char_to_str(ptr: *const c_char) -> Option<&'static str> {
    if ptr.is_null() {
        return None;
    }

    let cstr = CStr::from_ptr(ptr);
    let result = cstr.to_str();

    if result.is_err() {
        error!(
            "Unable to create string for '{}': {}",
            String::from_utf8_lossy(cstr.to_bytes()),
            result.err().unwrap()
        );

        return None;
    }

    Some(result.unwrap())
}

pub unsafe fn string_to_c_char(str: String) -> *const libc::c_char {
    let string_result = std::ffi::CString::new(str.as_bytes());

    if string_result.is_err() {
        error!(
            "Cannot create c string {}: {}",
            str,
            string_result.err().unwrap()
        );

        return null();
    }

    let data= Box::into_raw(Box::new(string_result.unwrap()));

    (&*data).as_ptr()
}
