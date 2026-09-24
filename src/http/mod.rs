mod addr;
mod header;
mod host;
mod query;
mod request;

#[cfg(not(target_arch = "wasm32"))]
pub mod ffi;

pub use addr::Addr;
pub use header::Header;
pub use host::{is_default_port, strip_default_port, without_default_port};
pub use query::{PathAndQueryWithSkipped, sanitize_url};
pub use request::Request;
