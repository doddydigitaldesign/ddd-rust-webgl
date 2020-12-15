use std::sync::Arc;
use std::sync::Mutex;

lazy_static! {
    static ref APP_STATE: Mutex<Arc<AppState>> = Mutex::new(Arc::new(AppState::new()));
}

pub fn update_dynamic_data(time: f32, canvas_height: f32, canvas_width: f32) {
    let min_height_width = canvas_height.min(canvas_width);
    let display_size = 0.9 * min_height_width;
    let half_display_size = display_size / 2.;
    let half_canvas_height = canvas_height / 2.;
    let half_canvas_width = canvas_width / 2.;

    let mut data = APP_STATE.lock().unwrap();

    *data = Arc::new(AppState {
        canvas_height: canvas_height,
        canvas_width: canvas_width,

        anchor_bottom: half_canvas_height - half_display_size,
        anchor_top: half_canvas_height + half_display_size,
        anchor_left: half_canvas_width - half_display_size,
        anchor_right: half_canvas_width + half_display_size,

        time: time,
        ..*data.clone()
    });
}

pub fn get_state() -> Arc<AppState> {
    APP_STATE.lock().unwrap().clone()
}

pub struct AppState {
    pub canvas_height: f32,
    pub canvas_width: f32,
    pub anchor_bottom: f32,
    pub anchor_top: f32,
    pub anchor_left: f32,
    pub anchor_right: f32,
    pub mouse_down: bool,
    pub mouse_x: f32,
    pub mouse_y: f32,
    pub rotation_x: f32,
    pub rotation_y: f32,
    pub time: f32,
    pub dt: f32,
}

impl AppState {
    fn new() -> Self {
        Self {
            canvas_height: 0.0,
            canvas_width: 0.0,
            anchor_bottom: 0.0,
            anchor_top: 0.0,
            anchor_left: 0.0,
            anchor_right: 0.0,
            mouse_down: false,
            mouse_x: -1.0,
            mouse_y: -1.0,
            rotation_x: -0.5,
            rotation_y: -0.5,
            time: 0.0,
            dt: 0.0,
        }
    }
}

pub fn update_mouse_down(x: f32, y: f32, is_down: bool) {
    let mut data = APP_STATE.lock().unwrap();
    *data = Arc::new(AppState {
        mouse_down: is_down,
        mouse_x: x,
        mouse_y: data.canvas_height - y,
        ..*data.clone()
    });
}

pub fn update_mouse_position(x: f32, y: f32) {
    let mut data = APP_STATE.lock().unwrap();
    let inverted_y = data.canvas_height - y;
    let x_delta = x - data.mouse_x;
    let y_delta = inverted_y - data.mouse_y;
    let rotation_x_delta = if data.mouse_down {
        std::f32::consts::PI * y_delta / data.canvas_height
    } else {
        0.
    };
    let rotation_y_delta = if data.mouse_down {
        std::f32::consts::PI * x_delta / data.canvas_width
    } else {
        0.
    };

    *data = Arc::new(AppState {
        mouse_x: x,
        mouse_y: inverted_y,
        rotation_x: data.rotation_x + rotation_x_delta,
        rotation_y: data.rotation_y - rotation_y_delta,
        ..*data.clone()
    });
}
