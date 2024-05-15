use godot::{engine::{Control, IControl, OptionButton, SpinBox}, prelude::*};

use crate::{settings::ApproachMode, FLUX};

#[derive(GodotClass)]
#[class(base=Control)]
pub struct NoteSettings {
    base: Base<Control>,
    approach_type: Option<Gd<OptionButton>>,
    approach_rate: Option<Gd<SpinBox>>,
    approach_time: Option<Gd<SpinBox>>,
    approach_distance: Option<Gd<SpinBox>>,
}

#[godot_api]
impl IControl for NoteSettings {
    fn init(base: Base<Control>) -> Self {
        Self {
            base,
            approach_type: None,
            approach_distance: None,
            approach_rate: None,
            approach_time: None,
        }
    }

    fn enter_tree(&mut self) {
        let mut approach_type = self.base().get_node_as::<OptionButton>("GridContainer/Approach/Type");
        let mut approach_rate = self.base().get_node_as::<SpinBox>("GridContainer/Approach/Rate");
        let mut approach_distance = self.base().get_node_as::<SpinBox>("GridContainer/Approach/Distance");
        let mut approach_time = self.base().get_node_as::<SpinBox>("GridContainer/Approach/Time");
        
        approach_type.connect("item_selected".into(), self.base_mut().callable("update_type"));
        approach_distance.connect("value_changed".into(), self.base_mut().callable("update_ad"));
        approach_rate.connect("value_changed".into(), self.base_mut().callable("update_ar"));
        approach_time.connect("value_changed".into(), self.base_mut().callable("update_at"));

        self.approach_type = Some(approach_type.clone());
        self.approach_rate = Some(approach_rate.clone());
        self.approach_distance = Some(approach_distance.clone());
        self.approach_time = Some(approach_time.clone());

        self.update_settings();
    }
}

#[godot_api]
impl NoteSettings {
    #[func]
    fn update_settings(&mut self) {
        let settings = unsafe { FLUX.settings.clone().unwrap() };

        self.approach_rate.as_mut().unwrap().set_value(settings.note.approach_rate as f64);
        self.approach_distance.as_mut().unwrap().set_value(settings.note.approach_distance as f64);
        self.approach_time.as_mut().unwrap().set_value(settings.note.approach_time as f64);

        match settings.note.approach_mode  {
            ApproachMode::DistRate => {
                self.approach_type.as_mut().unwrap().select(0);
                self.approach_distance.as_mut().unwrap().set_editable(true);
                self.approach_rate.as_mut().unwrap().set_editable(true);
                self.approach_time.as_mut().unwrap().set_editable(false);
            },
            ApproachMode::RateTime => {
                self.approach_type.as_mut().unwrap().select(1);
                self.approach_rate.as_mut().unwrap().set_editable(true);
                self.approach_time.as_mut().unwrap().set_editable(true);
                self.approach_distance.as_mut().unwrap().set_editable(false);
            },
            ApproachMode::DistTime => {
                self.approach_type.as_mut().unwrap().select(2);
                self.approach_distance.as_mut().unwrap().set_editable(true);
                self.approach_time.as_mut().unwrap().set_editable(true);
                self.approach_rate.as_mut().unwrap().set_editable(false);
            },
        }
    }

    #[func]
    fn update_type(&mut self, index: i32) {
        unsafe { FLUX.settings.as_mut().unwrap().note.approach_mode = match index {
            0 => ApproachMode::DistRate,
            1 => ApproachMode::RateTime,
            2 => ApproachMode::DistTime,
            _ => ApproachMode::DistTime,
        }};

        unsafe { FLUX.settings.as_mut().unwrap().update(false) }
        self.update_settings();
    }

    #[func]
    fn update_ar(&mut self, value: f64) {
        unsafe {
            FLUX.settings.as_mut().unwrap().note.approach_rate = value as f32;
            FLUX.settings.as_mut().unwrap().update(false);
        }
        self.update_settings();
    }
    
    #[func]
    fn update_ad(&mut self, value: f64) {
        unsafe { 
            FLUX.settings.as_mut().unwrap().note.approach_distance = value as f32;
            FLUX.settings.as_mut().unwrap().update(false);
        }
        self.update_settings();
    }

    #[func]
    fn update_at(&mut self, value: f64) {
        unsafe { 
            FLUX.settings.as_mut().unwrap().note.approach_time = value as f32;
            FLUX.settings.as_mut().unwrap().update(false);
        }
        self.update_settings();
    }
}