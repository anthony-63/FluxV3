use godot::prelude::*;

static HIT_WINDOW: f64 = 0.058;
static AABB: f64 = (1.75 + 0.525) / 2.;

#[derive(GodotClass)]
#[class(base=RefCounted)]
pub struct Note {
    base: Base<RefCounted>,
    pub x: f64,
    pub y: f64, 
    pub time: f64,
    pub hit: bool,
    pub index: usize,
    pub color: Color,
}

#[godot_api]
impl IRefCounted for Note {
    fn init(base: Base<RefCounted>) -> Self {
        Self {
            base,
            time: 0.,
            x: 0.,
            y: 0.,
            hit: false,
            index: 0,
            color: Color::from_html("#ffffff").unwrap(),
        }
    }
}

#[godot_api]
impl Note {
    pub fn in_hit_window(&self, note_time: f64, speed: f32) -> bool {
        return (note_time - self.time as f64) <= HIT_WINDOW * speed as f64;
    }

    pub fn is_visible(&self, note_time: f64, speed: f32, approach_time: f64) -> bool {
        if self.hit { return false; }
        // if note_time > self.time { return false; }
        return self.calculate_time(note_time, approach_time) <= 1. && self.in_hit_window(note_time, speed);
    }

    pub fn calculate_time(&self, note_time: f64, approach_time: f64) -> f64 {
        return (self.time - note_time) / approach_time;
    }

    pub fn is_touching(&self, cursor_pos: Vector2) -> bool {
        return (cursor_pos.x as f64 - self.x * 2.).abs() <= AABB && (cursor_pos.y as f64 - self.y * 2.).abs() <= AABB;
    }
}
