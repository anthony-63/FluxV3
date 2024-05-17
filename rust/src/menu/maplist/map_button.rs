use godot::{engine::{global::Error, Button, IButton, Image, ImageTexture, Label, TextureRect}, prelude::*};

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
        let run_selected_map = self.base_mut().callable("run_selected_map");
        self.base_mut().connect("pressed".into(), run_selected_map);
    }
}

#[godot_api]
impl MapButton {
    #[func]
    pub fn set_data(&mut self, map: Gd<Beatmap>, mapset: Gd<BeatmapSet>) {
        self.map = Some(map.clone());
        self.mapset = Some(mapset.clone());

        let mut titlelabel = self.base().get_node_as::<Label>("ArtistSongName");
        let mut difficultylabel = self.base().get_node_as::<Label>("Difficulty");
        let mut mapperlabel = self.base().get_node_as::<Label>("Mapper");
        let mut coverimage = self.base().get_node_as::<TextureRect>("Cover");

        titlelabel.set_text(mapset.bind().title.to_godot());
        difficultylabel.set_text(map.bind().name.to_godot());
        mapperlabel.set_text(mapset.bind().mappers.join(",").to_godot());
        let cover = mapset.bind().cover.clone();
        if  cover.is_some() {
            let bytes = cover.unwrap();
            let mut img = Image::new_gd();
            if img.load_png_from_buffer(bytes.as_slice().into()) != Error::OK {
                godot_warn!("failed to load png cover, skipping");
                return;
            }
            let texture = ImageTexture::create_from_image(img).unwrap();
            coverimage.set_texture(texture.upcast());
        }
    }

    #[func]
    pub fn run_selected_map(&mut self) {
        let mapset = self.mapset.clone().to_variant();
        let map = self.map.clone().to_variant();
        self.base_mut().emit_signal("selected_map".into(), &[mapset, map]);
    }

    #[signal]
    pub fn selected_map(mapset: Gd<BeatmapSet>, map: Gd<Beatmap>);
}