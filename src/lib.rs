/*!
This crate provides a library for matching, handling and logging http requests with reidrectionio
rule format.
*/

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate libc;

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

#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
