use wasm_bindgen::prelude::*;
use web_sys::console;

use crate::log;
#[wasm_bindgen]
pub struct App {
    foo: f64,
}

#[wasm_bindgen]
impl App {
    #[wasm_bindgen(constructor)]
    pub fn new() -> App {
        log("Client.new() was hit.");
        App { foo: 42.0 }
    }

    pub fn update(&mut self, _time: f32, _height: f32, _width: f32) -> Result<(), JsValue> {
        log("Client.update() was hit.");
        Ok(())
    }

    pub fn render(&self) {
        log("Client.render() was hit.");
    }
}
