
mod utils;
use wasm_bindgen::prelude::*;
use std::fmt;
use js_sys::Math;

mod opcodes;
pub mod cpu;

#[macro_use]
extern crate lazy_static;

extern crate web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


// Public methods, exported to JavaScript.
//#[wasm_bindgen]

//wasm-pack build --debug
//wc -c