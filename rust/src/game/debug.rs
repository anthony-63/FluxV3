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
        let binding = self.game.as_ref().unwrap();
        let game = binding.bind();
        let game_time = game.sync_manager.as_ref().unwrap().bind().real_time;
        drop(game);

        self.base_mut().set_text(format!(
            "{} FPS
Time: {:.2}",
        fps_monitor,
        game_time,
        ).to_godot());
    }
}