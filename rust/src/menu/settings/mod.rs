use godot::{engine::{AnimationPlayer, Button, Control, IControl}, prelude::*};

pub mod note;
pub mod audio;
pub mod cursor;

#[derive(GodotClass)]
#[class(base=Control)]
pub struct SettingsMenu {
    base: Base<Control>,
    pub closing: bool,
}

#[godot_api]
impl IControl for SettingsMenu {
    fn init(base: Base<Control>) -> Self {
        Self {
            base,
            closing: false,
        }
    }

    fn enter_tree(&mut self) {
        let mut close = self.base_mut().get_node_as::<Button>("Settings/Close");
        close.connect("pressed".into(), self.base_mut().callable("emit_close_settings"));

        let mut animation_player = self.base_mut().get_node_as::<AnimationPlayer>("AnimationPlayer");
        animation_player.connect("animation_finished".into(), self.base_mut().callable("finished_anim"));
    }
}

#[godot_api]
impl SettingsMenu {
    #[func]
    fn emit_close_settings(&mut self) {
        self.base_mut().emit_signal("close_settings".into(), &[]);
    }

    #[func]
    fn finished_anim(&mut self, _: StringName) {
        if self.closing {
            self.base_mut().set_visible(false);
        }
    }

    #[signal]
    fn close_settings();
}