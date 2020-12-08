use wasm_bindgen::prelude::*;
use web_sys::console;
#[wasm_bindgen]
pub struct Client {}

#[wasm_bindgen]
impl Client {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console::log_1(&JsValue::from_str("New was hit!"));
        Self {}
    }

    pub fn update(&mut self, _time: f32, _height: f32, _width: f32) -> Result<(), JsValue> {
        Ok(())
    }

    pub fn render(&self) {}
}
