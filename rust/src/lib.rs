use flux::Flux;
use game::mods::{speed::SpeedMod, AllMods};
use godot::prelude::*;

pub mod flux;
pub mod startup;
pub mod content;
pub mod menu;
pub mod settings;
pub mod game;

struct MyExtension;

const STAGE2_MAP_SEED: i64 = 1775906128188434359;

static mut FLUX: Flux = Flux {
    loaded_mapsets: vec![],
    settings: None,
    selected_map: None,
    selected_mapset: None,
    score: None,
    fullscreen: false,
    should_open_details: false,
    
    covers_instance_holder: vec![],

    mods: AllMods {
        speed: SpeedMod {
            value: 1.,
            enabled: false,
        }
    }
};

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}