use web_sys::WebGlRenderingContext as GL;
use web_sys::*;

pub fn compile_shader(
    gl: &WebGlRenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = gl
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Could not compile shader."))?;
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if gl
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(gl
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Could not get shader info log.")))
    }
}

pub fn link_program(
    gl: &WebGlRenderingContext,
    vertex_source: &str,
    fragment_source: &str,
) -> Result<WebGlProgram, String> {
    let program = gl
        .create_program()
        .ok_or_else(|| String::from("Could not create program"))?;

    let vertex_shader = compile_shader(&gl, GL::VERTEX_SHADER, vertex_source).unwrap();

    let fragment_shader = compile_shader(&gl, GL::FRAGMENT_SHADER, fragment_source).unwrap();

    gl.attach_shader(&program, &vertex_shader);
    gl.attach_shader(&program, &fragment_shader);
    gl.link_program(&program);

    if gl
        .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(gl
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Could not get program info log.")))
    }
}

/// Creates an array with 16 elements,
/// where every fifth element corresponds to a
/// new matrix row
pub fn translation_matrix(x: f32, y: f32, z: f32) -> [f32; 16] {
    let mut tmp = [0.0; 16];

    tmp[0] = 1.0;
    tmp[5] = 1.0;
    tmp[10] = 1.0;
    tmp[12] = x;
    tmp[13] = y;
    tmp[14] = z;

    tmp
}

/// Creates an array with 16 elements,
/// where every fifth element corresponds to a
/// new matrix row
pub fn scaling_matrix(x: f32, y: f32, z: f32) -> [f32; 16] {
    let mut tmp = [0.0; 16];

    tmp[0] = x;
    tmp[5] = y;
    tmp[10] = z;
    tmp[15] = 1.0;

    tmp
}

/// Multiplies two arrays representing 4x4 matrices
pub fn multiply_matrix_4(first: [f32; 16], second: [f32; 16]) -> [f32; 16] {
    let mut tmp = [0.0; 16];

    tmp[0] =
        first[0] * second[0] + first[1] * second[4] + first[2] * second[8] + first[3] * second[12];
    tmp[1] =
        first[0] * second[1] + first[1] * second[5] + first[2] * second[9] + first[3] * second[13];
    tmp[2] =
        first[0] * second[2] + first[1] * second[6] + first[2] * second[10] + first[3] * second[14];
    tmp[3] =
        first[0] * second[3] + first[1] * second[7] + first[2] * second[11] + first[3] * second[15];

    tmp[4] =
        first[4] * second[0] + first[5] * second[4] + first[6] * second[8] + first[7] * second[12];
    tmp[5] =
        first[4] * second[1] + first[5] * second[5] + first[6] * second[9] + first[7] * second[13];
    tmp[6] =
        first[4] * second[2] + first[5] * second[6] + first[6] * second[10] + first[7] * second[14];
    tmp[7] =
        first[4] * second[3] + first[5] * second[7] + first[6] * second[11] + first[7] * second[15];

    tmp[8] = first[8] * second[0]
        + first[9] * second[4]
        + first[10] * second[8]
        + first[11] * second[12];
    tmp[9] = first[8] * second[1]
        + first[9] * second[5]
        + first[10] * second[9]
        + first[11] * second[13];
    tmp[10] = first[8] * second[2]
        + first[9] * second[6]
        + first[10] * second[10]
        + first[11] * second[14];
    tmp[11] = first[8] * second[3]
        + first[9] * second[7]
        + first[10] * second[11]
        + first[11] * second[15];

    tmp[12] = first[12] * second[0]
        + first[13] * second[4]
        + first[14] * second[8]
        + first[15] * second[12];
    tmp[13] = first[12] * second[1]
        + first[13] * second[5]
        + first[14] * second[9]
        + first[15] * second[13];
    tmp[14] = first[12] * second[2]
        + first[13] * second[6]
        + first[14] * second[10]
        + first[15] * second[14];
    tmp[15] = first[12] * second[3]
        + first[13] * second[7]
        + first[14] * second[11]
        + first[15] * second[15];

    tmp
}
