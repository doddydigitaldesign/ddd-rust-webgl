use wasm_bindgen::prelude::*;
use web_sys::WebGlRenderingContext as GL;

use crate::{
    initialize_webgl_context, programs,
    state::{get_state, update_state},
};
use programs::{Color2D, Color2DGradient};
#[wasm_bindgen]
pub struct App {
    gl: GL,
    program_color_2d: Color2D,
    program_color_2d_gradient: Color2DGradient,
}

#[wasm_bindgen]
impl App {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        let gl = initialize_webgl_context().unwrap();
        Self {
            program_color_2d: Color2D::new(&gl),
            program_color_2d_gradient: Color2DGradient::new(&gl),
            gl: gl,
        }
    }

    pub fn update(
        &mut self,
        time: f32,
        canvas_height: f32,
        canvas_width: f32,
    ) -> Result<(), JsValue> {
        update_state(time, canvas_height, canvas_width);

        Ok(())
    }

    pub fn render(&self) {
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        let state = get_state();

        self.program_color_2d.render(
            &self.gl,
            state.anchor_bottom,
            state.anchor_top,
            state.anchor_left,
            state.anchor_right,
            state.canvas_height,
            state.canvas_width,
        );

        self.program_color_2d_gradient.render(
            &self.gl,
            state.anchor_bottom + 30.0,
            state.anchor_top - 30.0,
            state.anchor_left + 30.0,
            state.anchor_right - 30.0,
            state.canvas_height,
            state.canvas_width,
        );
    }
}
