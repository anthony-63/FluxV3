use godot::{engine::{EditorPlugin, IEditorPlugin}, prelude::*};

use crate::content::maps::mapset::Mapset;

#[derive(GodotClass)]
#[class(init, base=Object)]
pub struct Flux {
    base: Base<Object>,
    loaded_mapsets: Vec<Mapset>,
}

#[godot_api]
impl Flux {
    
}