use godot::{engine::{Button, Control, IControl}, prelude::*};

pub mod note;
pub mod audio;
pub mod cursor;

#[derive(GodotClass)]
#[class(base=Control)]
pub struct SettingsMenu {
    base: Base<Control>,
}

#[godot_api]
impl IControl for SettingsMenu {
    fn init(base: Base<Control>) -> Self {
        Self {
            base,
        }
    }

    fn enter_tree(&mut self) {
        let mut close = self.base_mut().get_node_as::<Button>("Settings/Close");
        close.connect("pressed".into(), self.base_mut().callable("emit_close_settings"));
    }
}

#[godot_api]
impl SettingsMenu {
    #[func]
    fn emit_close_settings(&mut self) {
        self.base_mut().emit_signal("close_settings".into(), &[]);
    }

    #[signal]
    fn close_settings();
}