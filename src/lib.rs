/*!
This crate provides a library for matching, handling and logging http requests with redirectionio
rule format.
*/

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

pub mod action;
pub mod api;
pub mod filter;
pub mod html;
pub mod http;
pub mod marker;
#[cfg(feature = "router")]
pub mod regex_radix_tree;
#[cfg(feature = "router")]
pub mod router;

#[cfg(not(target_arch = "wasm32"))]
mod callback_log;
#[cfg(feature = "dot")]
mod dot;
#[cfg(not(target_arch = "wasm32"))]
mod ffi_helpers;
mod regex;
mod router_config;

pub use router_config::RouterConfig;
