use godot::{engine::{performance::Monitor, ILabel, Label, Performance}, prelude::*};

use super::Game;

#[derive(GodotClass)]
#[class(base=Label)]
pub struct DebugLabel {
    base: Base<Label>,
    game: Option<Gd<Game>>,
}

#[godot_api]
impl ILabel for DebugLabel {
    fn init(base: Base<Label>) -> Self {
        Self {
            base,
            game: None,
        }
    }

    fn process(&mut self, _: f64) {
        if self.game.is_none() {
            let game = self.base().get_node_as::<Game>("../GameManager");
            self.game = Some(game);
        }

        if !self.game.clone().unwrap().bind().started {
            return;
        }

        let fps_monitor = Performance::singleton().get_monitor(Monitor::TIME_FPS);
        let game = self.game.as_ref().unwrap().bind();
        let game_time = game.sync_manager.as_ref().unwrap().bind().real_time;
        let notes_processed = game.note_manager.as_ref().unwrap().bind().notes_processing;
        let start_process = game.note_manager.as_ref().unwrap().bind().start_process;
        let cursor_pos = game.cursor.as_ref().unwrap().bind().clamped_position;
        let camera_yaw = game.cursor.as_ref().unwrap().bind().yaw;
        let camera_pitch = game.cursor.as_ref().unwrap().bind().pitch;

        drop(game);

        self.base_mut().set_text(format!(
            "{} FPS
Time: {:.2}
Processing: {}
Start Proc: {}
Camera Rot: ({}, {})
Cursor Pos: ({}, {})",
        fps_monitor,
        game_time,
        notes_processed,
        start_process,
        camera_pitch, camera_yaw,
        cursor_pos.x, cursor_pos.y,
        ).to_godot());
    }
}