use godot::{engine::{CheckButton, Control, IControl, SpinBox}, prelude::*};

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
        let mut absolute_scale_spinbox = self.base_mut().get_node_as::<SpinBox>("GridContainer/Cursor/AbsoluteScale");
        let mut parallax_spinbox = self.base_mut().get_node_as::<SpinBox>("GridContainer/Cursor/Parallax");
        let mut spin_checkbox = self.base_mut().get_node_as::<CheckButton>("GridContainer/Cursor/Spin");
        let mut absolute_checkbox = self.base_mut().get_node_as::<CheckButton>("GridContainer/Cursor/AbsoluteMode");

        let sensitivty = unsafe {FLUX.settings.as_ref().unwrap().cursor.sensitivity};
        let parallax = unsafe {FLUX.settings.as_ref().unwrap().camera.parallax};
        let spin = unsafe {FLUX.settings.as_ref().unwrap().camera.spin};

        let absolute = unsafe {FLUX.settings.as_ref().unwrap().cursor.absolute};
        let absolute_scale = unsafe {FLUX.settings.as_ref().unwrap().cursor.absolute_scale};

        sensitivity_spinbox.set_value(sensitivty as f64);
        absolute_scale_spinbox.set_value(absolute_scale as f64);
        parallax_spinbox.set_value(parallax as f64);
        spin_checkbox.set_pressed(spin);
        absolute_checkbox.set_pressed(absolute);

        sensitivity_spinbox.connect("value_changed".into(), self.base_mut().callable("change_sensitivity"));
        absolute_scale_spinbox.connect("value_changed".into(), self.base_mut().callable("change_absolute_scale"));
        parallax_spinbox.connect("value_changed".into(), self.base_mut().callable("change_parallax"));
        spin_checkbox.connect("toggled".into(), self.base_mut().callable("change_spin"));
        absolute_checkbox.connect("toggled".into(), self.base_mut().callable("change_absolute_mode"));
    }
}

#[godot_api]
impl CursorSettings {
    #[func]
    fn change_absolute_scale(&mut self, value: f64) {
        unsafe {
            FLUX.settings.as_mut().unwrap().cursor.absolute_scale = value as f32;
            FLUX.settings.as_mut().unwrap().update(false);
        }
    }

    #[func]
    fn change_sensitivity(&mut self, value: f64) {
        unsafe {
            FLUX.settings.as_mut().unwrap().cursor.sensitivity = value as f32;
            FLUX.settings.as_mut().unwrap().update(false);
        }
    }

    #[func]
    fn change_parallax(&mut self, value: f64) {
        unsafe {
            FLUX.settings.as_mut().unwrap().camera.parallax = value as f32;
            FLUX.settings.as_mut().unwrap().update(false);
        }
    }

    #[func]
    fn change_spin(&mut self, pressed: bool) {
        unsafe {
            FLUX.settings.as_mut().unwrap().camera.spin = pressed;
            FLUX.settings.as_mut().unwrap().update(false);
        }
    }

    #[func]
    fn change_absolute_mode(&mut self, pressed: bool) {
        unsafe {
            FLUX.settings.as_mut().unwrap().cursor.absolute = pressed;
            FLUX.settings.as_mut().unwrap().update(false);
        }
    }
}