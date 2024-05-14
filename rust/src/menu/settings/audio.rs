use godot::{engine::{Control, HSlider, IControl, Label}, prelude::*};

use crate::FLUX;

#[derive(GodotClass)]
#[class(base=Control)]
pub struct AudioSettings {
    base: Base<Control>,
    master_slider: Option<Gd<HSlider>>,
    master_label: Option<Gd<Label>>,
    music_slider: Option<Gd<HSlider>>,
    music_label: Option<Gd<Label>>,

}

#[godot_api]
impl IControl for AudioSettings {
    fn init(base: Base<Control>) -> Self {
        Self {
            base,
            master_label: None,
            music_label: None,
            master_slider: None,
            music_slider: None,
        }
    }

    fn enter_tree(&mut self) {
        let mut master_slider = self.base_mut().get_node_as::<HSlider>("GridContainer/Volumes/Master");
        let mut music_slider = self.base_mut().get_node_as::<HSlider>("GridContainer/Volumes/Music");

        let master_vol = unsafe {FLUX.settings.as_ref().unwrap().audio.master_volume};
        let music_vol = unsafe {FLUX.settings.as_ref().unwrap().audio.music_volume};

        self.master_label = Some(master_slider.get_node_as::<Label>("Percent"));
        self.music_label = Some(music_slider.get_node_as::<Label>("Percent"));
        self.master_slider = Some(master_slider.clone());
        self.music_slider = Some(music_slider.clone());

        master_slider.set_value(master_vol as f64);
        music_slider.set_value(music_vol as f64);

        self.master_label.as_mut().unwrap().set_text(format!("{}%", (master_vol * 100.).ceil()).into());
        self.music_label.as_mut().unwrap().set_text(format!("{}%", (music_vol * 100.).ceil()).into());

        master_slider.connect("value_changed".into(), self.base_mut().callable("change_master_volume"));
        music_slider.connect("value_changed".into(), self.base_mut().callable("change_music_volume"));
    }
}

#[godot_api]
impl AudioSettings {
    #[func]
    fn change_master_volume(&mut self, value: f64) {
        unsafe {
            FLUX.settings.as_mut().unwrap().audio.master_volume = value as f32;
            FLUX.settings.as_mut().unwrap().update(false);
            self.master_label.as_mut().unwrap().set_text(format!("{}%", (value * 100.).ceil()).into());
        }
    }

    #[func]
    fn change_music_volume(&mut self, value: f64) {
        unsafe {
            FLUX.settings.as_mut().unwrap().audio.music_volume = value as f32;
            FLUX.settings.as_mut().unwrap().update(false);
            self.music_label.as_mut().unwrap().set_text(format!("{}%", (value * 100.).ceil()).into());
        }
    }
}