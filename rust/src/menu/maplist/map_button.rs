use godot::{engine::{Button, IButton, Label}, prelude::*};

use crate::content::maps::{beatmap::Beatmap, beatmapset::BeatmapSet};

#[derive(GodotClass)]
#[class(base=Button)]
pub struct MapButton {
    base: Base<Button>,
    mapset: Option<Gd<BeatmapSet>>,
    map: Option<Gd<Beatmap>>,
}

#[godot_api]
impl IButton for MapButton {
    fn init(base: Base<Button>) -> Self {
        Self {
            base,
            map: None,
            mapset: None,
        }
    }

    fn enter_tree(&mut self,) {
    }
}

#[godot_api]
impl MapButton {
    #[func]
    fn set_data(&mut self, map: Gd<Beatmap>, beatmapset: Gd<BeatmapSet>) {
        self.map = Some(map.clone());
        self.mapset = Some(beatmapset.clone());

        let mut titlelabel = self.base().get_node_as::<Label>("ArtistSongName");
        let mut difficultylabel = self.base().get_node_as::<Label>("Difficulty");
        let mut mapperlabel = self.base().get_node_as::<Label>("Mapper");

        titlelabel.set_text(beatmapset.bind().title.to_godot());
        difficultylabel.set_text(map.bind().name.to_godot());
        mapperlabel.set_text(beatmapset.bind().mappers.join(",").to_godot());
    }
}