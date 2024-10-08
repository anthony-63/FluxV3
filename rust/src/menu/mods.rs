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
        speed_spinbox.set_value(unsafe { FLUX.game.mods.speed.value as f64 * 100. });
        speed_spinbox.connect("value_changed".into(), self.base_mut().callable("change_speed"));

        let mut toggle_speed = speed_spinbox.get_node_as::<Button>("Toggle");
        toggle_speed.set_pressed(unsafe { FLUX.game.mods.speed.enabled });
        toggle_speed.connect("toggled".into(), self.base_mut().callable("toggle_speed"));


        let mut ghost_spinbox = self.base().get_node_as::<SpinBox>("VBoxContainer/Ghost");
        ghost_spinbox.set_value(unsafe { FLUX.game.mods.ghost.value as f64 });
        ghost_spinbox.connect("value_changed".into(), self.base_mut().callable("change_ghost"));

        let mut toggle_ghost = ghost_spinbox.get_node_as::<Button>("Toggle");
        toggle_ghost.set_pressed(unsafe { FLUX.game.mods.ghost.enabled });
        toggle_ghost.connect("toggled".into(), self.base_mut().callable("toggle_ghost"));


        let mut toggle_nofail = self.base().get_node_as::<Button>("VBoxContainer/NoFail");
        toggle_nofail.set_pressed(unsafe { FLUX.game.mods.nofail.enabled });
        toggle_nofail.connect("toggled".into(), self.base_mut().callable("toggle_nofail"));
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
            FLUX.game.mods.speed.enabled = toggled;
            let mut audio_player = self.base().get_node_as::<AudioStreamPlayer>("../../../Music");
            if toggled {
                audio_player.set_pitch_scale(FLUX.game.mods.speed.value);
            } else {
                audio_player.set_pitch_scale(1.);
            }
        }
    }

    #[func]
    fn change_speed(&mut self, value: f64) {
        unsafe {
            FLUX.game.mods.speed.value = (value / 100.) as f32;
            if FLUX.game.mods.speed.enabled {
                let mut audio_player = self.base().get_node_as::<AudioStreamPlayer>("../../../Music");
                audio_player.set_pitch_scale(FLUX.game.mods.speed.value);
            }
        }
    }

    #[func]
    fn toggle_ghost(&mut self, toggled: bool) {
        unsafe {
            FLUX.game.mods.ghost.enabled = toggled;
        }
    }

    #[func]
    fn change_ghost(&mut self, value: f64) {
        unsafe {
            FLUX.game.mods.ghost.value = value as f32;
        }
    }

    #[func]
    fn toggle_nofail(&mut self, toggled: bool) {
        unsafe {
            FLUX.game.mods.nofail.enabled = toggled;
        }
    }
}