use crate::constants::GRID_SIZE;

use super::super::util;
use js_sys::WebAssembly;
use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext as GL;
use web_sys::*;

pub struct Graph3D {
    pub program: WebGlProgram,
    pub indices_buffer: WebGlBuffer,
    pub index_count: i32,
    pub normals_buffer: WebGlBuffer,
    pub position_buffer: WebGlBuffer,
    pub u_normals_rotation: WebGlUniformLocation,
    pub u_projection: WebGlUniformLocation,
    pub y_buffer: WebGlBuffer,
    pub time_color_blue: WebGlUniformLocation,
    pub time_color_red: WebGlUniformLocation,
    pub time_color_green: WebGlUniformLocation,
}

impl Graph3D {
    pub fn new(gl: &GL) -> Self {
        let program = util::link_program(
            &gl,
            &super::super::shaders::vertex::graph_3d::SHADER,
            &super::super::shaders::fragment::graph_3d::SHADER,
        )
        .unwrap();

        let positions_and_indices = util::get_position_grid_n_by_n(GRID_SIZE);
        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .unwrap()
            .buffer();
        let vertices_location = positions_and_indices.0.as_ptr() as u32 / 4;
        let vert_array = js_sys::Float32Array::new(&memory_buffer).subarray(
            vertices_location,
            vertices_location + positions_and_indices.0.len() as u32,
        );
        let buffer_position = gl.create_buffer().ok_or("failed to create buffer").unwrap();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer_position));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vert_array, GL::STATIC_DRAW);

        let indices_memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .unwrap()
            .buffer();
        let indices_location = positions_and_indices.1.as_ptr() as u32 / 2;
        let indices_array = js_sys::Uint16Array::new(&indices_memory_buffer).subarray(
            indices_location,
            indices_location + positions_and_indices.1.len() as u32,
        );
        let buffer_indices = gl.create_buffer().unwrap();
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&buffer_indices));
        gl.buffer_data_with_array_buffer_view(
            GL::ELEMENT_ARRAY_BUFFER,
            &indices_array,
            GL::STATIC_DRAW,
        );

        Self {
            u_normals_rotation: gl
                .get_uniform_location(&program, "uNormalsRotation")
                .unwrap(),
            u_projection: gl.get_uniform_location(&program, "uProjection").unwrap(),
            time_color_blue: gl
                .get_uniform_location(&program, "time_color_blue")
                .unwrap(),
            time_color_red: gl.get_uniform_location(&program, "time_color_red").unwrap(),
            time_color_green: gl
                .get_uniform_location(&program, "time_color_green")
                .unwrap(),

            program: program,

            indices_buffer: buffer_indices,
            index_count: indices_array.length() as i32,
            normals_buffer: gl
                .create_buffer()
                .ok_or("failed normals create buffer")
                .unwrap(),
            position_buffer: buffer_position,
            y_buffer: gl.create_buffer().ok_or("failed to create buffer").unwrap(),
        }
    }

    pub fn render(
        &self,
        gl: &GL,
        bottom: f32,
        top: f32,
        left: f32,
        right: f32,
        canvas_height: f32,
        canvas_width: f32,
        rotation_x: f32,
        rotation_y: f32,
        y_values: &Vec<f32>,
        time: f32,
    ) {
        gl.use_program(Some(&self.program));

        let half_grid = (GRID_SIZE + 1) / 2;
        let last_index = GRID_SIZE - 1;

        let red_index = (((time / 1000.0) / 2.0).floor() as i32 % (half_grid / 2) as i32) as usize;
        let green_index = (((time / 1000.0) / 3.0).floor() as i32 % half_grid as i32) as usize;
        let blue_index = (((time / 1000.0) / 7.0).floor() as i32 % last_index as i32) as usize;

        let time_color_red = y_values[red_index] + 0.5;
        let time_color_green = y_values[green_index] + 0.5;
        let time_color_blue = y_values[blue_index] + 0.5;

        gl.uniform1f(Some(&self.time_color_blue), time_color_blue);
        gl.uniform1f(Some(&self.time_color_red), time_color_red);
        gl.uniform1f(Some(&self.time_color_green), time_color_green);

        let matrices = util::get_3d_matrices(
            bottom,
            top,
            left,
            right,
            canvas_height,
            canvas_width,
            rotation_x,
            rotation_y,
        );

        gl.uniform_matrix4fv_with_f32_array(Some(&self.u_projection), false, &matrices.projection);
        gl.uniform_matrix4fv_with_f32_array(
            Some(&self.u_normals_rotation),
            false,
            &matrices.normals_rotation,
        );

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.position_buffer));
        gl.vertex_attrib_pointer_with_i32(0, 3, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(0);

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.y_buffer));
        gl.vertex_attrib_pointer_with_i32(1, 1, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(1);

        let y_memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .unwrap()
            .buffer();
        let y_pointer = y_values.as_ptr() as u32 / 4;
        let y_array = js_sys::Float32Array::new(&y_memory_buffer)
            .subarray(y_pointer, y_pointer + y_values.len() as u32);
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &y_array, GL::DYNAMIC_DRAW);

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.normals_buffer));
        gl.vertex_attrib_pointer_with_i32(2, 3, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(2);

        let normals = util::get_grid_normals(super::super::constants::GRID_SIZE, &y_values);
        let normals_vals_memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .unwrap()
            .buffer();
        let normals_vals_location = normals.as_ptr() as u32 / 4;
        let normals_vals_array = js_sys::Float32Array::new(&normals_vals_memory_buffer).subarray(
            normals_vals_location,
            normals_vals_location + normals.len() as u32,
        );
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.normals_buffer));
        gl.buffer_data_with_array_buffer_view(
            GL::ARRAY_BUFFER,
            &normals_vals_array,
            GL::DYNAMIC_DRAW,
        );

        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&self.indices_buffer));

        gl.draw_elements_with_i32(GL::TRIANGLES, self.index_count, GL::UNSIGNED_SHORT, 0);
    }
}
