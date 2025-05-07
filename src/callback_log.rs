use std::{
    os::raw::{c_char, c_short, c_void},
    sync::Once,
};

use log::{Metadata, Record};

use crate::ffi_helpers::string_to_c_char;

#[allow(non_camel_case_types)]
pub type redirectionio_log_callback = extern "C" fn(*const c_char, *const c_void, c_short);

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
            let cstr = string_to_c_char(log_str);

            (self.callback.unwrap())(cstr, self.data.unwrap(), record.level() as i16);
        }
    }

    fn flush(&self) {}
}

static INIT: Once = Once::new();

#[unsafe(no_mangle)]
pub extern "C" fn redirectionio_log_init_stderr() {
    stderrlog::new().init().unwrap();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn redirectionio_log_init_with_callback(callback: redirectionio_log_callback, data: &'static c_void) {
    let logger = CallbackLogger {
        callback: Some(callback),
        data: Some(data),
    };

    INIT.call_once(|| {
        log::set_boxed_logger(Box::new(logger))
            .map(|()| log::set_max_level(log::LevelFilter::Trace))
            .expect("cannot set logger");
    });
}
