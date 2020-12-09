use wasm_bindgen::prelude::*;
use web_sys::console;

mod app;
pub use app::*;
mod gl_setup;
pub use gl_setup::*;
mod core;
pub use crate::core::{programs, shaders};
mod state;
use state::{get_state, update_state};
mod util;

#[macro_use]
extern crate lazy_static;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // Your code goes here!

    Ok(())
}

#[wasm_bindgen]
pub fn log(msg: &str) {
    unsafe {
        console::log_1(&JsValue::from_str(msg));
    }
}
