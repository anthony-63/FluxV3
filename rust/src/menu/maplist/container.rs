use godot::{engine::{IVBoxContainer, VBoxContainer}, prelude::*};

use crate::{content::maps::{beatmap::Beatmap, beatmapset::BeatmapSet}, FLUX};

use super::map_button::MapButton;

#[derive(GodotClass)]
#[class(base=VBoxContainer)]
pub struct MapContainer {
    base: Base<VBoxContainer>,
    audio_player: Option<Gd<AudioStreamPlayer>>,
}

#[godot_api]
impl IVBoxContainer for MapContainer {
    fn init(base: Base<VBoxContainer>) -> Self {
        Self {
            base,
            audio_player: None,
        }
    }

    fn enter_tree(&mut self) {
        self.audio_player = Some(self.base().get_node_as::<AudioStreamPlayer>("../../Music"));

        let entry_prefab = load::<PackedScene>("res://prefabs/map_button.tscn");
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
}

#[godot_api]
impl MapContainer {
    #[func]
    pub fn selected_map(&mut self, mapset: Gd<BeatmapSet>, _map: Gd<Beatmap>) {
        let map_audio = mapset.bind().load_audio(true);
        if map_audio == None {
            return;
        }

        self.audio_player.as_mut().unwrap().set_stream(map_audio.unwrap());
        self.audio_player.as_mut().unwrap().play();
    }
}