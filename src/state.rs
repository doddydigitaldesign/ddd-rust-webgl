use std::sync::{Arc, Mutex};

lazy_static! {
    static ref STATE: Mutex<Arc<State>> = Mutex::new(Arc::new(State::new()));
}

pub fn update_state(time: f32, canvas_height: f32, canvas_width: f32) {
    let min_height_width = canvas_height.min(canvas_width);

    let display_size = 0.9 * min_height_width;

    let half_display_size = display_size / 2.0;

    let half_canvas_height = canvas_height / 2.0;

    let half_canvas_width = canvas_width / 2.0;

    let mut data = STATE.lock().unwrap();

    *data = Arc::new(State {
        canvas_height,
        canvas_width,

        anchor_bottom: half_canvas_height - half_display_size,
        anchor_top: half_canvas_height + half_display_size,
        anchor_left: half_canvas_width - half_display_size,
        anchor_right: half_canvas_width + half_display_size,

        time,
        ..*data.clone()
    });
}

pub fn get_state() -> Arc<State> {
    STATE.lock().unwrap().clone()
}

pub struct State {
    pub canvas_height: f32,
    pub canvas_width: f32,
    pub time: f32,
    pub anchor_bottom: f32,
    pub anchor_top: f32,
    pub anchor_left: f32,
    pub anchor_right: f32,
}

impl State {
    fn new() -> Self {
        Self {
            canvas_height: 0.0,
            canvas_width: 0.0,
            time: 0.0,
            anchor_bottom: 0.0,
            anchor_top: 0.0,
            anchor_left: 0.0,
            anchor_right: 0.0,
        }
    }
}
