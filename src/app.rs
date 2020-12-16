use wasm_bindgen::prelude::*;
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;

use crate::{gl_setup, programs, state, util};

#[wasm_bindgen]
pub struct App {
    gl: WebGlRenderingContext,
    program_graph_3d: programs::Graph3D,
}

#[wasm_bindgen]
impl App {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        let gl = gl_setup::initialize_webgl_context().unwrap();

        Self {
            program_graph_3d: programs::Graph3D::new(&gl),
            gl,
        }
    }

    pub fn update(&mut self, time: f32, dt: f32, height: f32, width: f32) -> Result<(), JsValue> {
        state::update_dynamic_data(time, dt, height, width);
        Ok(())
    }

    pub fn render(&self) {
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        let app_state = state::get_state();

        let y_values = util::get_solved_equation(app_state.time, app_state.dt);

        self.program_graph_3d.render(
            &self.gl,
            app_state.anchor_bottom,
            app_state.anchor_top,
            app_state.anchor_left,
            app_state.anchor_right,
            app_state.canvas_height,
            app_state.canvas_width,
            app_state.rotation_x,
            app_state.rotation_y,
            &y_values,
        );
    }
}
