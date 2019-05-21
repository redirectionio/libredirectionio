use log::{Metadata, Record};
#[cfg(not(target_arch = "wasm32"))]
use std::intrinsics::transmute;
#[cfg(not(target_arch = "wasm32"))]
use std::ptr::null;

#[allow(non_camel_case_types)]
pub type redirectionio_log_callback = extern "C" fn(*const libc::c_char, *const libc::c_void);

pub struct CallbackLogger {
    pub callback: Option<redirectionio_log_callback>,
    pub data: Option<&'static libc::c_void>,
}

impl log::Log for CallbackLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        return true;
    }

    fn log(&self, record: &Record) {
        if self.callback.is_none() {
            return;
        }

        if self.data.is_none() {
            return;
        }

        if self.enabled(record.metadata()) {
            let log_str = format!("{} - {}", record.level(), record.args());
            let cstr = safe_str_to_cstr(log_str);

            (self.callback.unwrap())(cstr, self.data.unwrap());
        }
    }

    fn flush(&self) {}
}

#[cfg(not(target_arch = "wasm32"))]
fn safe_str_to_cstr(str: String) -> *const libc::c_char {
    unsafe {
        let string_result = std::ffi::CString::new(str.as_bytes());

        if string_result.is_err() {
            return null();
        }

        let data: *const std::ffi::CString = transmute(Box::new(string_result.unwrap()));

        return (&*data).as_ptr();
    };
}
