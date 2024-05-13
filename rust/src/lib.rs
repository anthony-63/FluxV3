use flux::Flux;
use godot::{engine::Engine, prelude::*};

pub mod flux;
pub mod startup;
pub mod content;

struct MyExtension;

static mut FLUX: Flux = Flux {
    loaded_mapsets: vec![],
};

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}