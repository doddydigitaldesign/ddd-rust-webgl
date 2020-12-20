use crate::log;

use super::constants::*;

extern crate nalgebra;
use nalgebra::{Matrix4, Perspective3};
use web_sys::WebGl2RenderingContext as GL;
use web_sys::*;

pub fn get_grid_normals(n: usize, y_values: &Vec<f32>) -> Vec<f32> {
    let points_per_row = n + 1;
    let graph_layout_width: f32 = 2.;
    let square_size: f32 = graph_layout_width / n as f32;
    let mut tmp: Vec<f32> = vec![0.0; 3 * points_per_row.pow(2)];

    for z in 0..points_per_row {
        for x in 0..points_per_row {
            let y_a = z * points_per_row + x;
            let start_index = 3 * y_a;

            if z == n || x == n {
                tmp[start_index + 1] = 1.0;
            } else {
                let y_b = y_a + points_per_row;
                let y_c = y_a + 1;

                let x_1 = square_size * x as f32;
                let x_2 = x_1 + square_size;

                let z_1 = square_size * z as f32;
                let z_2 = z_1 + square_size;

                let normals = get_normal_vec(
                    x_1,
                    y_values[y_a],
                    z_1,
                    x_1,
                    y_values[y_b],
                    z_2,
                    x_2,
                    y_values[y_c],
                    z_2,
                );

                tmp[start_index + 0] = normals.0;
                tmp[start_index + 1] = normals.1;
                tmp[start_index + 2] = normals.2;
            }
        }
    }

    tmp
}

pub fn get_normal_vec(
    a_x: f32,
    a_y: f32,
    a_z: f32,
    b_x: f32,
    b_y: f32,
    b_z: f32,
    c_x: f32,
    c_y: f32,
    c_z: f32,
) -> (f32, f32, f32) {
    let u_x = b_x - a_x;
    let u_y = b_y - a_y;
    let u_z = b_z - a_z;

    let v_x = c_x - a_x;
    let v_y = c_y - a_y;
    let v_z = c_z - a_z;

    let normal_x = u_y * v_z - v_y * u_z;
    let normal_y = -1.0 * (u_x * v_z - v_x * u_z);
    let normal_z = u_x * v_y - v_x * u_y;

    let normal_size = (normal_x.powi(2) + normal_y.powi(2) + normal_z.powi(2)).sqrt();

    (
        normal_x / normal_size,
        normal_y / normal_size,
        normal_z / normal_size,
    )
}

pub fn get_solved_equation(curr_time: f32, _time_diff: f32) -> Vec<f32> {
    let row_size = GRID_SIZE + 1;
    let mut y_values: Vec<f32> = vec![0.0; row_size.pow(2)];
    let half_grid: f32 = row_size as f32 / 2.0;
    let _dt = _time_diff / 1000.0;
    let sin_offset: f32 = curr_time / 1000.0; //speed

    for z in 0..row_size {
        for x in 0..row_size {
            let y_index = z * row_size + x;
            let scaled_z = FREQUENCY_SCALE * (z as f32 - half_grid) / half_grid;
            let scaled_x = FREQUENCY_SCALE * (x as f32 - half_grid) / half_grid;

            y_values[y_index] =
                Y_SCALE * ((scaled_x.powi(2) + scaled_z.powi(2)).sqrt() - sin_offset).sin();
        }
    }

    y_values
}

pub struct Matrices3D {
    pub normals_rotation: [f32; 16],
    pub projection: [f32; 16],
}

pub fn get_3d_matrices(
    bottom: f32,
    top: f32,
    left: f32,
    right: f32,
    canvas_height: f32,
    canvas_width: f32,
    rotation_angle_x_axis: f32,
    rotation_angle_y_axis: f32,
) -> Matrices3D {
    let mut return_var = Matrices3D {
        normals_rotation: [0.0; 16],
        projection: [0.0; 16],
    };

    let rotate_x_axis: [f32; 16] = [
        1.0,
        0.0,
        0.0,
        0.0,
        0.0,
        rotation_angle_x_axis.cos(),
        -rotation_angle_x_axis.sin(),
        0.0,
        0.0,
        rotation_angle_x_axis.sin(),
        rotation_angle_x_axis.cos(),
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
    ];

    let rotate_y_axis: [f32; 16] = [
        rotation_angle_y_axis.cos(),
        0.0,
        rotation_angle_y_axis.sin(),
        0.0,
        0.0,
        1.0,
        0.0,
        0.0,
        -rotation_angle_y_axis.sin(),
        0.0,
        rotation_angle_y_axis.cos(),
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
    ];

    let rotation_matrix = mult_matrix_4(rotate_x_axis, rotate_y_axis);

    let aspect: f32 = canvas_width / canvas_height;
    let scale_x = (right - left) / canvas_width;
    let scale_y = (top - bottom) / canvas_height;
    let scale = (scale_x + scale_y) / 2.0;

    let translation_matrix: [f32; 16] = translation_matrix(
        -1.0 + scale_x + 2.0 * left / canvas_width,
        -1.0 + scale_y + 2.0 * bottom / canvas_height,
        Z_PLANE,
    );

    let scale_matrix: [f32; 16] = scaling_matrix(scale, scale, 0.0);
    let rotation_scale = mult_matrix_4(rotation_matrix, scale_matrix);
    let combined_transform = mult_matrix_4(rotation_scale, translation_matrix);
    let perspective_matrix_tmp: Perspective3<f32> =
        Perspective3::new(aspect, FIELD_OF_VIEW, Z_NEAR, Z_FAR);
    let mut perspective: [f32; 16] = [0.0; 16];
    perspective.copy_from_slice(perspective_matrix_tmp.as_matrix().as_slice());

    return_var.projection = mult_matrix_4(combined_transform, perspective);

    let normal_matrix = Matrix4::new(
        rotation_matrix[0],
        rotation_matrix[1],
        rotation_matrix[2],
        rotation_matrix[3],
        rotation_matrix[4],
        rotation_matrix[5],
        rotation_matrix[6],
        rotation_matrix[7],
        rotation_matrix[8],
        rotation_matrix[9],
        rotation_matrix[10],
        rotation_matrix[11],
        rotation_matrix[12],
        rotation_matrix[13],
        rotation_matrix[14],
        rotation_matrix[15],
    );

    match normal_matrix.try_inverse() {
        Some(inv) => {
            return_var.normals_rotation.copy_from_slice(inv.as_slice());
        }
        None => {}
    }

    return_var
}

pub fn get_position_grid_n_by_n(n: usize) -> (Vec<f32>, Vec<u16>) {
    let n_plus_one = n + 1;
    let mut positions: Vec<f32> = vec![0.0; 3 * n_plus_one * n_plus_one];
    let mut indices: Vec<u16> = vec![0; 6 * n * n];

    let graph_layout_width: f32 = 2.0;
    let square_size: f32 = graph_layout_width / n as f32;

    for z in 0..n_plus_one {
        for x in 0..n_plus_one {
            let start_pos_i = 3 * (z * n_plus_one + x);
            positions[start_pos_i + 0] = -1.0 + (x as f32) * square_size;
            positions[start_pos_i + 1] = 0.0;
            positions[start_pos_i + 2] = -1.0 + (z as f32) * square_size;

            if z < n && x < n {
                let start_index_i = 6 * (z * n + x);
                let vertex_index_top_left = (z * n_plus_one + x) as u16;
                let vertex_index_bottom_left = vertex_index_top_left + n_plus_one as u16;
                let vertex_index_top_right = vertex_index_top_left + 1;
                let vertex_index_bottom_right = vertex_index_bottom_left + 1;

                indices[start_index_i + 0] = vertex_index_top_left;
                indices[start_index_i + 1] = vertex_index_bottom_left;
                indices[start_index_i + 2] = vertex_index_bottom_right;
                indices[start_index_i + 3] = vertex_index_top_left;
                indices[start_index_i + 4] = vertex_index_bottom_right;
                indices[start_index_i + 5] = vertex_index_top_right;
            }
        }
    }

    (positions, indices)
}

pub fn link_program(gl: &GL, vert_source: &str, frag_source: &str) -> Result<WebGlProgram, String> {
    let program = gl
        .create_program()
        .ok_or_else(|| String::from("Error creating program"))?;

    let vert_shader = compile_shader(&gl, GL::VERTEX_SHADER, vert_source).unwrap();

    let frag_shader = compile_shader(&gl, GL::FRAGMENT_SHADER, frag_source).unwrap();

    gl.attach_shader(&program, &vert_shader);
    gl.attach_shader(&program, &frag_shader);
    gl.link_program(&program);

    if gl
        .get_program_parameter(&program, GL::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(gl
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}

fn compile_shader(gl: &GL, shader_type: u32, source: &str) -> Result<WebGlShader, String> {
    let shader = gl
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Error creating shader"))?;
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if gl
        .get_shader_parameter(&shader, GL::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(gl
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unable to get shader info log")))
    }
}

pub fn translation_matrix(x: f32, y: f32, z: f32) -> [f32; 16] {
    let mut tmp = [0.0; 16];

    tmp[0] = 1.0;
    tmp[5] = 1.0;
    tmp[10] = 1.0;
    tmp[15] = 1.0;

    tmp[12] = x;
    tmp[13] = y;
    tmp[14] = z;

    tmp
}

pub fn scaling_matrix(x: f32, y: f32, z: f32) -> [f32; 16] {
    let mut tmp = [0.0; 16];

    tmp[0] = x;
    tmp[5] = y;
    tmp[10] = z;
    tmp[15] = 1.0;

    tmp
}

pub fn mult_matrix_4(a: [f32; 16], b: [f32; 16]) -> [f32; 16] {
    let mut tmp = [0.0; 16];

    tmp[0] = a[0] * b[0] + a[1] * b[4] + a[2] * b[8] + a[3] * b[12];
    tmp[1] = a[0] * b[1] + a[1] * b[5] + a[2] * b[9] + a[3] * b[13];
    tmp[2] = a[0] * b[2] + a[1] * b[6] + a[2] * b[10] + a[3] * b[14];
    tmp[3] = a[0] * b[3] + a[1] * b[7] + a[2] * b[11] + a[3] * b[15];

    tmp[4] = a[4] * b[0] + a[5] * b[4] + a[6] * b[8] + a[7] * b[12];
    tmp[5] = a[4] * b[1] + a[5] * b[5] + a[6] * b[9] + a[7] * b[13];
    tmp[6] = a[4] * b[2] + a[5] * b[6] + a[6] * b[10] + a[7] * b[14];
    tmp[7] = a[4] * b[3] + a[5] * b[7] + a[6] * b[11] + a[7] * b[15];

    tmp[8] = a[8] * b[0] + a[9] * b[4] + a[10] * b[8] + a[11] * b[12];
    tmp[9] = a[8] * b[1] + a[9] * b[5] + a[10] * b[9] + a[11] * b[13];
    tmp[10] = a[8] * b[2] + a[9] * b[6] + a[10] * b[10] + a[11] * b[14];
    tmp[11] = a[8] * b[3] + a[9] * b[7] + a[10] * b[11] + a[11] * b[15];

    tmp[12] = a[12] * b[0] + a[13] * b[4] + a[14] * b[8] + a[15] * b[12];
    tmp[13] = a[12] * b[1] + a[13] * b[5] + a[14] * b[9] + a[15] * b[13];
    tmp[14] = a[12] * b[2] + a[13] * b[6] + a[14] * b[10] + a[15] * b[14];
    tmp[15] = a[12] * b[3] + a[13] * b[7] + a[14] * b[11] + a[15] * b[15];

    tmp
}
