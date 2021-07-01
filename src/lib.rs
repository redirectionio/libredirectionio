#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate libc;
extern crate wasm_bindgen;

pub mod action;
pub mod api;
pub mod filter;
pub mod html;
pub mod http;
pub mod regex_radix_tree;
pub mod router;

#[cfg(not(target_arch = "wasm32"))]
mod callback_log;
#[cfg(not(target_arch = "wasm32"))]
mod ffi;
#[cfg(not(target_arch = "wasm32"))]
mod ffi_helpers;
