use godot::{engine::{EditorPlugin, IEditorPlugin}, prelude::*};

use crate::content::maps::beatmapset::BeatmapSet;

pub struct Flux {
    pub loaded_mapsets: Vec<BeatmapSet>,
}
