use godot::{engine::{Button, IPanel, Panel, SpinBox}, prelude::*};

use crate::FLUX;

#[derive(GodotClass)]
#[class(base=Panel)]
pub struct ModPanel {
    base: Base<Panel>,
}

#[godot_api]
impl IPanel for ModPanel {
    fn init(base: Base<Panel>) -> Self {
        Self {
            base,
        }
    }

    fn enter_tree(&mut self) {
        let mut close_button = self.base().get_node_as::<Button>("Close");
        close_button.connect("pressed".into(), self.base_mut().callable("hide_self"));
        
        let mut speed_spinbox = self.base().get_node_as::<SpinBox>("VBoxContainer/Speed");
        speed_spinbox.set_value(unsafe { FLUX.mods.speed.value as f64 * 100. });
        speed_spinbox.connect("value_changed".into(), self.base_mut().callable("change_speed"));

        let mut toggle_speed = speed_spinbox.get_node_as::<Button>("Toggle");
        toggle_speed.set_pressed(unsafe { FLUX.mods.speed.enabled });
        toggle_speed.connect("toggled".into(), self.base_mut().callable("toggle_speed"));
    }

    fn process(&mut self, _: f64) {

    }
}

#[godot_api]
impl ModPanel {
    #[func]
    fn hide_self(&mut self) {
        self.base_mut().set_visible(false);
    }

    #[func]
    fn toggle_speed(&mut self, toggled: bool) {
        unsafe {
            FLUX.mods.speed.enabled = toggled;
        }
    }

    #[func]
    fn change_speed(&mut self, value: f64) {
        unsafe {
            FLUX.mods.speed.value = (value / 100.) as f32;
        }
    }
}