use flux::Flux;
use game::mods::{ghost::GhostMod, nofail::NoFailMod, speed::SpeedMod, AllMods};
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
    total_diff_count: 0,

    settings: None,

    selected_map: None,
    selected_mapset: None,

    score: None,

    fullscreen: false,
    should_open_details: false,

    discord_client: None,

    start_from: 0.,

    covers_instance_holder: vec![],

    mods: AllMods {
        speed: SpeedMod {
            value: 1.,
            enabled: false,
        },
        nofail: NoFailMod {
            enabled: false,
        },
        ghost: GhostMod {
            enabled: false,
            value: 32.,
        }
    }
};

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}