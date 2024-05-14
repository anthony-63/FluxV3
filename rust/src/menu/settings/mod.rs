use godot::{engine::{Control, IControl}, prelude::*};

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
}

#[godot_api]
impl SettingsMenu {
}