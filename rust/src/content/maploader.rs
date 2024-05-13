use godot::{engine::{Os, Time}, prelude::*};

use crate::{content::maps::beatmapset::BeatmapSet, FLUX};

pub struct MapLoader;
impl MapLoader {
    pub fn load_all_from_dir(path: String) {
        godot_print!("loading all maps from {}", path);

        let map_folders = std::fs::read_dir(path).unwrap();

        for folder in map_folders {
            unsafe {
                FLUX.loaded_mapsets.push(BeatmapSet::from_folder(folder.unwrap().path().to_str().unwrap().to_string()));
            }
        }
    }
}