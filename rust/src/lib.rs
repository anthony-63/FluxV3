use flux::Flux;
use godot::prelude::*;

pub mod flux;
pub mod startup;
pub mod content;
pub mod menu;
pub mod settings;
pub mod game;

struct MyExtension;

static mut FLUX: Flux = Flux {
    loaded_mapsets: vec![],
    settings: None,
    selected_map: None,
    selected_mapset: None,
};

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}