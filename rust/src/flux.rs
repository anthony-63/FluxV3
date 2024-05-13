use godot::{engine::{EditorPlugin, IEditorPlugin}, prelude::*};

use crate::content::maps::mapset::Mapset;

pub struct Flux {
    pub loaded_mapsets: Vec<Mapset>,
}
