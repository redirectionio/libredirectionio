mod header;
mod request;
mod query;

#[cfg(not(target_arch = "wasm32"))]
pub mod ffi;
pub mod wasm;

pub use header::Header;
pub use request::Request;

use lazy_static::lazy_static;
pub use query::QueryParamSkipBuilder;

lazy_static! {
    pub static ref STATIC_QUERY_PARAM_SKIP_BUILDER: QueryParamSkipBuilder = {
        QueryParamSkipBuilder::default()
    };
}
