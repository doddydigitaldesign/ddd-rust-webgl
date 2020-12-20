pub const ANGLE_TO_RADIAN_FACTOR: f32 = std::f32::consts::PI / 180.0;
pub const FIELD_OF_VIEW: f32 = 90.0 * ANGLE_TO_RADIAN_FACTOR;
pub const GRID_SIZE: usize = 255;
pub const Z_FAR: f32 = 100.0;
pub const Z_NEAR: f32 = 0.1;
pub const Z_PLANE: f32 = -1.0 / (std::f32::consts::PI / 4.0);
pub const FREQUENCY_SCALE: f32 = 5.0 * std::f32::consts::PI;
pub const Y_SCALE: f32 = FREQUENCY_SCALE * ANGLE_TO_RADIAN_FACTOR / 10.0;
