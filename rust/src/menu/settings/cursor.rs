use godot::{engine::{Control, IControl, SpinBox}, prelude::*};

use crate::FLUX;

#[derive(GodotClass)]
#[class(base=Control)]
pub struct CursorSettings {
    base: Base<Control>,
}

#[godot_api]
impl IControl for CursorSettings {
    fn init(base: Base<Control>) -> Self {
        Self {
            base,
        }
    }

    fn enter_tree(&mut self) {
        let mut sensitivity_spinbox = self.base_mut().get_node_as::<SpinBox>("GridContainer/Cursor/Sensitivity");

        let sensitivty = unsafe {FLUX.settings.as_ref().unwrap().cursor.sensitivity};

        sensitivity_spinbox.set_value(sensitivty as f64);
        sensitivity_spinbox.connect("value_changed".into(), self.base_mut().callable("change_sensitivity"));
    }
}

#[godot_api]
impl CursorSettings {
    #[func]
    fn change_sensitivity(&mut self, value: f64) {
        unsafe {
            FLUX.settings.as_mut().unwrap().cursor.sensitivity = value as f32;
            FLUX.settings.as_mut().unwrap().update(false);
        }
    }
}