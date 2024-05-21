use std::{sync::mpsc::{Receiver, Sender}, thread};

use godot::{engine::{global::Error, GridContainer, IGridContainer, Image, ImageTexture, LineEdit, TextureRect}, obj::WithBaseField, prelude::*};

use crate::{content::maps::{beatmap::Beatmap, beatmapset::BeatmapSet}, FLUX};

use super::{details::MapDetails, map_button::MapButton};

#[derive(GodotClass)]
#[class(base=GridContainer)]
pub struct MapContainer {
    base: Base<GridContainer>,
    audio_player: Option<Gd<AudioStreamPlayer>>,
    search_box: Option<Gd<LineEdit>>,
    map_details: Option<Gd<MapDetails>>,
    bg_blur: Option<Gd<TextureRect>>,

    cover_reciever: Option<Receiver<(String, InstanceId)>>, 
}

#[godot_api]
impl IGridContainer for MapContainer {
    fn init(base: Base<GridContainer>) -> Self {
        Self {
            base,
            audio_player: None,
            search_box: None,
            map_details: None,
            bg_blur: None,
            cover_reciever: None,
        }
    }

    fn enter_tree(&mut self) {
        self.audio_player = Some(self.base().get_node_as::<AudioStreamPlayer>("../../../../Music"));
        self.search_box = Some(self.base().get_node_as::<LineEdit>("../../Filters/Search"));
        self.map_details = Some(self.base().get_node_as::<MapDetails>("../../MapDetails"));
        self.bg_blur = Some(self.base().get_node_as::<TextureRect>("../../BgBlur"));

        let entry_prefab = load::<PackedScene>("res://prefabs/map_button.tscn");

        let (mut sender, reciever) = std::sync::mpsc::channel();
        self.cover_reciever = Some(reciever);

        thread::spawn(move || {
            Self::load_covers_threaded(&mut sender);
        });
        
        unsafe {
            for map in FLUX.loaded_mapsets.clone() {
                for diff in map.difficulties.clone() {
                    let mut entry = entry_prefab.instantiate_as::<MapButton>();
                    entry.call("set_data".into(), &[Gd::from_object(diff).to_variant(), Gd::from_object(map.clone()).to_variant()]);
                    entry.set_visible(true);
                    entry.connect("selected_map".into(), self.base_mut().callable("selected_map"));
                    self.base_mut().add_child(entry.upcast::<Node>());
                }
            }
        }
    }

    fn process(&mut self, _: f64) {
        if self.search_box.is_none() {
            return;
        }

        let search = self.search_box.as_ref().unwrap().get_text().to_string().to_lowercase();

        if self.cover_reciever.is_some() {
            match self.cover_reciever.as_ref().unwrap().try_recv() {
                Ok(cover) => {

                    for child_uncast in self.base().get_children().iter_shared() {
                        let mut child=  child_uncast.try_cast::<MapButton>().unwrap();
                        let mut button_ref = child.bind_mut();

                        if button_ref.mapset.as_ref().unwrap().bind().hash == cover.0 {
                            let texture: Gd<ImageTexture> = match Gd::try_from_instance_id(cover.1) {
                                Ok(tex) => tex,
                                Err(e) => {
                                    godot_print!("Cover instance error: {}", e);
                                    continue;
                                }
                            };
                            button_ref.set_cover(texture.clone());
                            godot_print!("{}: {}", button_ref.mapset.as_ref().unwrap().bind().title, cover.0);
                        }
                    }
                },
                Err(_) => {},
            }
        }

        for child_uncast in self.base().get_children().iter_shared() {
            let mut child=  child_uncast.try_cast::<MapButton>().unwrap();
            let button_ref = child.bind_mut();
            
            if search == "".to_string() {
                button_ref.to_gd().set_visible(true);
                continue;
            }

            let map = &button_ref.map.as_ref().unwrap().bind();
            let mapset = &button_ref.mapset.as_ref().unwrap().bind();

            let lower_mappers = mapset.mappers.iter().map(|x| x.to_lowercase()).collect::<Vec<String>>();
            let contains_mapper = lower_mappers.iter().map(|x| x.contains(&search)).collect::<Vec<bool>>().contains(&true);

            let visible = 
                mapset.title.to_lowercase().contains(&search) || 
                contains_mapper ||
                map.name.to_lowercase().contains(&search) ||
                mapset.artist.to_lowercase().contains(&search);

            button_ref.to_gd().set_visible(visible);
        }
    }
}

#[godot_api]
impl MapContainer {
    #[func]
    pub fn selected_map(&mut self, mapset: Gd<BeatmapSet>, map: Gd<Beatmap>) {
        unsafe {
            FLUX.selected_map = Some(map.clone());
            FLUX.selected_mapset = Some(mapset.clone());
        }
        // self.base_mut().get_tree().unwrap().change_scene_to_file("res://scenes/game.tscn".into_godot());

        let map_audio = mapset.bind().load_audio(true);
        if map_audio == None {
            return;
        }

        self.map_details.as_mut().unwrap().set_visible(true);
        self.bg_blur.as_mut().unwrap().set_visible(true);

        self.map_details.as_mut().unwrap().call("set_details".into(), &[]);

        self.audio_player.as_mut().unwrap().set_stream(map_audio.unwrap());
        self.audio_player.as_mut().unwrap().play();
    }

    pub fn load_covers_threaded(sender: &mut Sender<(String, InstanceId)>) {
        unsafe { FLUX.covers_instance_holder.clear() };

        for mapset in unsafe { FLUX.loaded_mapsets.clone() } {
            godot_print!("loading cover: {}", mapset.title);
            let cover = mapset.cover.as_ref();
            if  cover.is_some() {
                let bytes = cover.unwrap();
                let mut img = Image::new_gd();
                if img.load_png_from_buffer(bytes.as_slice().into()) != Error::OK {
                    godot_warn!("failed to load png cover, skipping");
                    return;
                }
                let texture = ImageTexture::create_from_image(img).unwrap();

                unsafe { FLUX.covers_instance_holder.push(texture.clone()) };

                let _ = sender.send((mapset.hash, texture.instance_id()));
            }
        }
    }
}