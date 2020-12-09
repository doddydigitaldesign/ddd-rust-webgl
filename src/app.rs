use wasm_bindgen::prelude::*;
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;

use crate::{initialize_webgl_context, log};
#[wasm_bindgen]
pub struct App {
    gl: GL,
}

#[wasm_bindgen]
impl App {
    #[wasm_bindgen(constructor)]
    pub fn new() -> App {
        console_error_panic_hook::set_once();
        let gl = initialize_webgl_context().unwrap();
        App { gl }
    }

    pub fn update(&mut self, _time: f32, _height: f32, _width: f32) -> Result<(), JsValue> {
        log("Client.update() was hit.");
        Ok(())
    }

    pub fn render(&self) {
        log("Client.render() was hit.");
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
    }
}
