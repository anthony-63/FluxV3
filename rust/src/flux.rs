use crate::{content::maps::beatmapset::BeatmapSet, settings::Settings};

pub struct Flux {
    pub loaded_mapsets: Vec<BeatmapSet>,
    pub settings: Option<Settings>,
}
