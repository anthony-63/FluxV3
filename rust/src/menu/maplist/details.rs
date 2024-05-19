use godot::{engine::{global::{Error, MouseButton}, Button, IPanel, Image, ImageTexture, InputEvent, InputEventMouseButton, Label, Panel, TextureRect}, prelude::*};

use crate::FLUX;

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

        let cursor_pos = event.get_global_position();
        if !Rect2::new(self.base().get_global_position(), self.base().get_size()).has_point(cursor_pos) {
            self.base_mut().set_visible(false);
            self.bg_blur.as_mut().unwrap().set_visible(false);
        }
    }
}

#[godot_api]
impl MapDetails {
    #[func]
    pub fn set_details(&mut self) {
        let mut title = self.base().get_node_as::<Label>("Details/VBoxContainer/Title");
        let mut mapper = self.base().get_node_as::<Label>("Details/VBoxContainer/Mapper");
        let mut difficulty = self.base().get_node_as::<Label>("Details/VBoxContainer/Difficulty");
        let mut notes = self.base().get_node_as::<Label>("Details/VBoxContainer/Notes");
        let mut cover_rect = self.base().get_node_as::<TextureRect>("Cover");

        let map = unsafe { FLUX.selected_map.as_ref().unwrap() };
        let mapset = unsafe { FLUX.selected_mapset.as_ref().unwrap() };

        title.set_text(mapset.bind().title.clone().into());
        mapper.set_text(mapset.bind().mappers.join(", ").clone().into());
        difficulty.set_text(map.bind().name.clone().into());
        notes.set_text(format!("{} notes", map.bind().notes.len()).into());
        
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
    }

    #[func]
    fn play_map(&mut self) {
        self.base_mut().get_tree().unwrap().change_scene_to_file("res://scenes/game.tscn".into_godot());
    }
}
