use log::{Metadata, Record};
use std::os::raw::{c_char, c_void, c_short};
use std::sync::Once;
use crate::ffi_helpers::string_to_c_char;

#[allow(non_camel_case_types)]
pub type redirectionio_log_callback = extern fn(*const c_char, *const c_void, c_short);

pub struct CallbackLogger {
    pub callback: Option<redirectionio_log_callback>,
    pub data: Option<&'static c_void>,
}

impl log::Log for CallbackLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
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
            let cstr = unsafe { string_to_c_char(log_str) };

            (self.callback.unwrap())(cstr, self.data.unwrap(), record.level() as i16);
        }
    }

    fn flush(&self) {}
}

static mut LOGGER: CallbackLogger = CallbackLogger {
    callback: None,
    data: None,
};

static INIT: Once = Once::new();

#[no_mangle]
pub extern fn redirectionio_log_init_stderr() {
    stderrlog::new().init().unwrap();
}

#[no_mangle]
pub unsafe extern fn redirectionio_log_init_with_callback(
    callback: redirectionio_log_callback,
    data: &'static libc::c_void,
) {
    LOGGER.callback = Some(callback);
    LOGGER.data = Some(data);

    INIT.call_once(|| {
        log::set_logger(&LOGGER)
            .map(|()| log::set_max_level(log::LevelFilter::Trace))
            .expect("cannot set logger");
    });
}
