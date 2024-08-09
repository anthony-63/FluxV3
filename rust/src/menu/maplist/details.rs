use godot::{engine::{global::{Error, MouseButton}, Button, HSlider, IPanel, Image, ImageTexture, InputEvent, InputEventMouseButton, Label, Panel, TextureRect, VBoxContainer}, prelude::*};

use crate::FLUX;

use crate::menu::mods::ModPanel;

#[derive(GodotClass)]
#[class(base=Panel)]
pub struct MapDetails {
    base: Base<Panel>,
    bg_blur: Option<Gd<TextureRect>>,
}

#[godot_api]
impl IPanel for MapDetails {
    fn init(base: Base<Panel>) -> Self {
        Self {
            base,
            bg_blur: None,
        }
    }

    fn enter_tree(&mut self) {
        self.bg_blur = Some(self.base().get_node_as::<TextureRect>("../BgBlur"));
        
        let mut open_mods = self.base().get_node_as::<Button>("Mods");
        open_mods.connect("pressed".into(), self.base_mut().callable("open_mods"));

        let mut close_details = self.bg_blur.as_ref().unwrap().get_node_as::<Button>("Close");
        close_details.connect("pressed".into(), self.base_mut().callable("close_details"));
        
        let mut play_button = self.base().get_node_as::<Button>("Play");
        play_button.connect("pressed".into(), self.base_mut().callable("play_map"));

        let mut start_from_slider = self.base().get_node_as::<HSlider>("StartFrom");
        start_from_slider.connect("value_changed".into(), self.base_mut().callable("update_start_from"));
    }

    fn process(&mut self, _: f64) {
    }
}

#[godot_api]
impl MapDetails {
    #[func]
    pub fn close_details(&mut self) {
        self.base_mut().set_visible(false);
        self.bg_blur.as_mut().unwrap().set_visible(false);
    }

    #[func]
    pub fn set_details(&mut self) {
        let mut title = self.base().get_node_as::<Label>("Details/VBoxContainer/Title");
        let mut mapper = self.base().get_node_as::<Label>("Details/VBoxContainer/Mapper");
        let mut difficulty = self.base().get_node_as::<Label>("Details/VBoxContainer/Difficulty");
        let mut notes = self.base().get_node_as::<Label>("Details/VBoxContainer/Notes");
        let mut length = self.base().get_node_as::<Label>("Details/VBoxContainer/Length");
        let mut cover_rect = self.base().get_node_as::<TextureRect>("Cover");
        
        let mut start_from_slider = self.base().get_node_as::<HSlider>("StartFrom");
        let mut start_from_label = self.base().get_node_as::<Label>("StartFrom/Count");

        let mut pb_no_score = self.base().get_node_as::<Label>("PB/NoScore");
        let mut pb_status = self.base().get_node_as::<Label>("PB/Status");
        let mut pb_rank = self.base().get_node_as::<Label>("PB/Rank");
        let mut score_info = self.base().get_node_as::<VBoxContainer>("PB/VBoxContainer");

        let map = unsafe { FLUX.game.selected_map.as_ref().unwrap() };
        let mapset = unsafe { FLUX.game.selected_mapset.as_ref().unwrap() };

        title.set_text(mapset.bind().title.clone().into());
        mapper.set_text(mapset.bind().mappers.join(", ").clone().into());
        difficulty.set_text(map.bind().name.clone().into());
        notes.set_text(format!("{} notes", map.bind().notes.len()).into());
        

        if map.bind().notes.len() > 0 {
            let last_note_time = map.bind().notes.last().unwrap().time;
            start_from_slider.set_max(last_note_time as f64);
            unsafe { 
                FLUX.game.start_from = FLUX.game.start_from.min(last_note_time as f64);
            }
            
            length.set_text(format!("{:01}:{:02}",
                            (last_note_time / 60.).floor() as usize,
                            (last_note_time % 60.).floor() as usize).as_str().into())
        } else {
            length.set_text("0:00".into());
            start_from_slider.set_max(0.);
        }
        
        start_from_slider.set_value(unsafe { FLUX.game.start_from });

        start_from_label.set_text(format!("{:01}:{:02}",
                                    (start_from_slider.get_value() / 60.).floor() as usize,
                                    (start_from_slider.get_value() % 60.).floor() as usize).as_str().into());
        
        let cover = mapset.bind().cover.clone();
        if cover.is_some() {
            let bytes = cover.unwrap();
            let mut img = Image::new_gd();
            if img.load_png_from_buffer(bytes.as_slice().into()) != Error::OK {
                godot_warn!("failed to load png cover, skipping");
                return;
            }

            let texture = ImageTexture::create_from_image(img).unwrap();
            
            cover_rect.set_texture(texture.upcast());
        } else {
            cover_rect.set_texture(load("res://assets/skins/Default/cover_placeholder.png"));
        }

        unsafe {
            if FLUX.game.score.is_some() && FLUX.game.score.as_ref().unwrap().map_id == FLUX.game.selected_map.as_ref().unwrap().bind().id {
                let mut accuracy = score_info.get_node_as::<Label>("Accuracy/Count");
                let mut misses = score_info.get_node_as::<Label>("Misses/Count");
                let mut mods = score_info.get_node_as::<Label>("Mods/List");
                
                let score = FLUX.game.score.as_ref().unwrap();

                pb_no_score.set_visible(false);
                pb_rank.set_visible(true);
                score_info.set_visible(true);

                accuracy.set_text(format!("{:.02}%", score.get_accuracy()).into());
                misses.set_text(format!("{}", score.misses).into());
                
                let mut mod_text = String::from("None");

                if score.mods_used.speed.enabled {
                    mod_text = format!("S{}", (score.mods_used.speed.value * 100.).floor());
                }

                mods.set_text(mod_text.into());

                pb_status.set_text(if score.failed {
                    pb_rank.set_text("F".into());
                    String::from("Failed at ") + format!("{:01}:{:02}",
                                                    (score.fail_time / 60.).floor() as usize,
                                                    (score.fail_time % 60.).floor() as usize).as_str()
                } else {
                    pb_rank.set_text(score.get_rank().into());
                    String::from("Passed")
                }.into());

            } else {
                pb_no_score.set_visible(true);
                pb_rank.set_visible(false);
                pb_status.set_visible(false);
                score_info.set_visible(false);
            }
        }
    }

    #[func]
    fn play_map(&mut self) {
        self.base_mut().get_tree().unwrap().change_scene_to_file("res://scenes/game.tscn".into_godot());
    }

    #[func]
    fn open_mods(&mut self) {
        self.base_mut().get_node_as::<ModPanel>("../ModPanel").set_visible(true);
    }

    #[func]
    fn update_start_from(&mut self, value: f64) {
        let mut start_from_label = self.base().get_node_as::<Label>("StartFrom/Count");
        let mut audio_player = self.base().get_node_as::<AudioStreamPlayer>("../../../Music");

        start_from_label.set_text(format!("{:01}:{:02}",
                                    (value / 60.).floor() as usize,
                                    (value % 60.).floor() as usize).as_str().into());
        unsafe { FLUX.game.start_from = value };
        audio_player.seek(value as f32);
    }
}
