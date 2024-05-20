use godot::obj::Gd;

use crate::{content::maps::{beatmap::Beatmap, beatmapset::BeatmapSet}, game::{mods::AllMods, score::Score}, settings::Settings};

pub struct Flux {
    pub loaded_mapsets: Vec<BeatmapSet>,
    pub settings: Option<Settings>,
    pub score: Option<Score>,

    pub fullscreen: bool,

    pub selected_mapset: Option<Gd<BeatmapSet>>,
    pub selected_map: Option<Gd<Beatmap>>,

    pub should_open_details: bool,

    pub mods: AllMods,
}