use std::{
    ffi::{CStr, CString},
    os::raw::c_char,
    ptr::null,
};

pub fn c_char_to_str<'a>(ptr: *const c_char) -> Option<&'a str> {
    if ptr.is_null() {
        return None;
    }

    // SAFETY: ptr is a valid pointer to a C string
    let cstr = unsafe { CStr::from_ptr(ptr) };

    match cstr.to_str() {
        Err(error) => {
            tracing::error!(
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
            tracing::error!("cannot create c string {str}: {error}");

            return null();
        }
        Ok(string) => string,
    };

    string.into_raw()
}

/// Free a string previously returned by one of the `redirectionio_*` functions.
///
/// Strings handed out across the FFI boundary are allocated with Rust's
/// allocator (`CString::into_raw`), so they must be reclaimed by Rust as well
/// rather than with the C `free()` function.
///
/// # Safety
///
/// This function must be called with a pointer returned by a `redirectionio_*`
/// function that returns a string, or a null pointer. Each such pointer must be
/// dropped at most once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn redirectionio_string_drop(string: *const c_char) {
    if string.is_null() {
        return;
    }

    // Safety: string was created with CString::into_raw in string_to_c_char
    drop(unsafe { CString::from_raw(string as *mut c_char) });
}
