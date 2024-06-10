use flux::{Flux, StaticGame, StaticMapList, StaticMaps};
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

    maps: StaticMaps {
        loaded_mapsets: vec![],
        total_diff_count: 0,
    },

    maplist: StaticMapList {
        should_open_details: false,
        covers_instance_holder: vec![], 
    },

    settings: None,

    game: StaticGame {
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
        },

        score: None,

        selected_map: None,
        selected_mapset: None,

        start_from: 0.,
    },


    fullscreen: false,

    discord_client: None,
};

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}