extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[macro_use]
extern crate lazy_static;

pub mod app;
mod constants;
mod gl_setup;
mod programs;
mod shaders;
mod state;
mod util;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
