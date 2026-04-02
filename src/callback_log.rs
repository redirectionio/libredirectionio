use std::os::raw::{c_char, c_short, c_void};
#[allow(non_camel_case_types)]
pub type redirectionio_log_callback = extern "C" fn(*const c_char, *const c_void, c_short);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn redirectionio_log_init_with_callback(_callback: redirectionio_log_callback, _data: &'static c_void) {
    // do nothing
    // bc layer
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn redirectionio_trace_init() {
    if let Err(e) = tracing_subscriber::fmt::try_init() {
        eprintln!("[redirectionio] unable to set global default subscriber: {e}");
    }
}
