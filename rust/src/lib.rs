use flux::Flux;
use godot::prelude::*;

pub mod flux;
pub mod startup;
pub mod content;
pub mod menu;
struct MyExtension;

static mut FLUX: Flux = Flux {
    loaded_mapsets: vec![],
};

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}