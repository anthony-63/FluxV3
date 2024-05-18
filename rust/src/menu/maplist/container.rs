use godot::{engine::{GridContainer, IGridContainer, LineEdit}, prelude::*};

use crate::{content::maps::{beatmap::Beatmap, beatmapset::BeatmapSet}, FLUX};

use super::map_button::MapButton;

#[derive(GodotClass)]
#[class(base=GridContainer)]
pub struct MapContainer {
    base: Base<GridContainer>,
    audio_player: Option<Gd<AudioStreamPlayer>>,
    search_box: Option<Gd<LineEdit>>,
}

#[godot_api]
impl IGridContainer for MapContainer {
    fn init(base: Base<GridContainer>) -> Self {
        Self {
            base,
            audio_player: None,
            search_box: None,
        }
    }

    fn enter_tree(&mut self) {
        self.audio_player = Some(self.base().get_node_as::<AudioStreamPlayer>("../../Music"));
        self.search_box = Some(self.base().get_node_as::<LineEdit>("../../Filters/Search"));

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

    fn process(&mut self, _: f64) {
        if self.search_box.is_none() {
            return;
        }

        let search = self.search_box.as_ref().unwrap().get_text().to_string().to_lowercase();

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
        self.base_mut().get_tree().unwrap().change_scene_to_file("res://scenes/game.tscn".into_godot());

        // let map_audio = mapset.bind().load_audio(true);
        // if map_audio == None {
        //     return;
        // }

        // self.audio_player.as_mut().unwrap().set_stream(map_audio.unwrap());
        // self.audio_player.as_mut().unwrap().play();
    }
}