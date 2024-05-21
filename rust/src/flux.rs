use godot::{engine::ImageTexture, obj::Gd};

use crate::{content::maps::{beatmap::Beatmap, beatmapset::BeatmapSet}, game::{mods::AllMods, score::Score}, settings::Settings};

pub struct Flux {
    pub loaded_mapsets: Vec<BeatmapSet>,

    pub total_diff_count: usize,

    pub settings: Option<Settings>,
    pub score: Option<Score>,

    pub fullscreen: bool,

    pub selected_mapset: Option<Gd<BeatmapSet>>,
    pub selected_map: Option<Gd<Beatmap>>,

    pub should_open_details: bool,

    pub mods: AllMods,

    pub covers_instance_holder: Vec<Gd<ImageTexture>>,
}