use super::super::super::shaders;
use super::super::super::util;
use js_sys::Float32Array;
use js_sys::WebAssembly;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;

pub struct Color2D {
    program: WebGlProgram,
    rect_vertices_array_length: usize,
    rect_vertices_buffer: WebGlBuffer,
    u_color: WebGlUniformLocation,
    u_opacity: WebGlUniformLocation,
    u_transform: WebGlUniformLocation,
}

impl Color2D {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        let program = util::link_program(
            &gl,
            shaders::vertex::color_2d::SHADER,
            shaders::fragment::color_2d::SHADER,
        )
        .unwrap();

        // vertices of triangles in counter-clockwise fashion
        let vertices_rect: [f32; 12] = [
            0.0, 1.0, // top-left
            0.0, 0.0, // bottom-left
            1.0, 1.0, // top-right
            1.0, 1.0, // top-right
            0.0, 0.0, // bottom-left
            1.0, 0.0, // bottom right
        ];

        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .unwrap()
            .buffer();

        let vertices_pointer = vertices_rect.as_ptr() as u32 / 4;

        let vertices_array = Float32Array::new(&memory_buffer).subarray(
            vertices_pointer,
            vertices_pointer + vertices_rect.len() as u32,
        );

        let buffer_rect = gl
            .create_buffer()
            .ok_or("Could not create buffer for rect.")
            .unwrap();

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer_rect));

        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vertices_array, GL::STATIC_DRAW);

        Self {
            u_color: gl.get_uniform_location(&program, "uColor").unwrap(),
            u_opacity: gl.get_uniform_location(&program, "uOpacity").unwrap(),
            u_transform: gl.get_uniform_location(&program, "uTransform").unwrap(),
            rect_vertices_array_length: vertices_rect.len(),
            rect_vertices_buffer: buffer_rect,
            program,
        }
    }

    pub fn render(
        &self,
        gl: &WebGlRenderingContext,
        bottom: f32,
        top: f32,
        left: f32,
        right: f32,
        canvas_height: f32,
        canvas_width: f32,
    ) {
        gl.use_program(Some(&self.program));

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.rect_vertices_buffer));

        gl.vertex_attrib_pointer_with_i32(0, 2, GL::FLOAT, false, 0, 0);

        gl.enable_vertex_attrib_array(0);

        // RGBA
        gl.uniform4f(Some(&self.u_color), 0.0, 0.5, 0.5, 1.0);

        gl.uniform1f(Some(&self.u_opacity), 1.0);

        let transform_translate = util::translation_matrix(
            2.0 * left / canvas_width - 1.0,
            2.0 * bottom / canvas_height - 1.0,
            0.0,
        );

        let transform_scale = util::scaling_matrix(
            2.0 * (right - left) / canvas_width,
            2.0 * (top - bottom) / canvas_height,
            0.0,
        );

        let transform_scale_translate =
            util::multiply_matrix_4(transform_scale, transform_translate);

        gl.uniform_matrix4fv_with_f32_array(
            Some(&self.u_transform),
            false,
            &transform_scale_translate,
        );

        gl.draw_arrays(
            GL::TRIANGLES,
            0,
            (self.rect_vertices_array_length / 2) as i32,
        );
    }
}
