use godot::{engine::{global::{Error, MouseButton}, Button, IPanel, Image, ImageTexture, InputEvent, InputEventMouseButton, Label, Panel, TextureRect}, prelude::*};

use crate::content::maps::{beatmap::Beatmap, beatmapset::BeatmapSet};

#[derive(GodotClass)]
#[class(base=Panel)]
pub struct MapDetails {
    base: Base<Panel>,
    title: Option<Gd<Label>>,
    mapper: Option<Gd<Label>>,
    difficulty: Option<Gd<Label>>,
    notes: Option<Gd<Label>>,
    cover: Option<Gd<TextureRect>>,
    bg_blur: Option<Gd<TextureRect>>,
}

#[godot_api]
impl IPanel for MapDetails {
    fn init(base: Base<Panel>) -> Self {
        Self {
            base,
            title: None,
            bg_blur: None,
            difficulty: None,
            mapper: None,
            notes: None,
            cover: None,
        }
    }

    fn enter_tree(&mut self) {
        self.title = Some(self.base().get_node_as::<Label>("Details/VBoxContainer/Title"));
        self.mapper = Some(self.base().get_node_as::<Label>("Details/VBoxContainer/Mapper"));
        self.difficulty = Some(self.base().get_node_as::<Label>("Details/VBoxContainer/Difficulty"));
        self.notes = Some(self.base().get_node_as::<Label>("Details/VBoxContainer/Notes"));
        self.cover = Some(self.base().get_node_as::<TextureRect>("Cover"));

        self.bg_blur = Some(self.base().get_node_as::<TextureRect>("../BgBlur"));
        
        let mut play_button = self.base().get_node_as::<Button>("Play");
        play_button.connect("pressed".into(), self.base_mut().callable("play_map"));
    }

    fn process(&mut self, _: f64) {
    }

    fn input(&mut self, ev: Gd<InputEvent>) {
        let Ok(event) = ev.try_cast::<InputEventMouseButton>() else {
            return;
        };

        if !event.is_pressed() || (event.get_button_index() != MouseButton::LEFT && event.get_button_index() != MouseButton::RIGHT) {
            return;
        }

        let cursor_pos = event.get_position();
        if !self.base().get_rect().has_point(cursor_pos) {
            self.base_mut().set_visible(false);
            self.bg_blur.as_mut().unwrap().set_visible(false);
        }
    }
}

#[godot_api]
impl MapDetails {
    #[func]
    pub fn set_details(&mut self, map: Gd<Beatmap>, mapset: Gd<BeatmapSet>) {
        self.title.as_mut().unwrap().set_text(mapset.bind().title.clone().into());
        self.mapper.as_mut().unwrap().set_text(mapset.bind().mappers.join(", ").clone().into());
        self.difficulty.as_mut().unwrap().set_text(map.bind().name.clone().into());
        self.notes.as_mut().unwrap().set_text(format!("{} notes", map.bind().notes.len()).into());
        
        let cover = mapset.bind().cover.clone();
        if cover.is_some() {
            let bytes = cover.unwrap();
            let mut img = Image::new_gd();
            if img.load_png_from_buffer(bytes.as_slice().into()) != Error::OK {
                godot_warn!("failed to load png cover, skipping");
                return;
            }

            let texture = ImageTexture::create_from_image(img).unwrap();
            
            self.cover.as_mut().unwrap().set_texture(texture.upcast());
        } else {
            self.cover.as_mut().unwrap().set_texture(load("res://assets/skins/Default/cover_placeholder.png"));
        }
    }

    #[func]
    fn play_map(&mut self) {
        self.base_mut().get_tree().unwrap().change_scene_to_file("res://scenes/game.tscn".into_godot());
    }
}