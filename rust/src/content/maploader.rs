use godot::prelude::*;

use crate::{content::maps::{beatmapset::BeatmapSet, sspm::SSPMParser}, FLUX};

pub struct MapLoader;
impl MapLoader {
    pub fn load_all_from_dir(path: String) {
        godot_print!("loading all maps from {}", path);

        let map_folders = std::fs::read_dir(path).unwrap();

        for filename in map_folders {
            let file = filename.unwrap();

            godot_print!("loading {}", file.path().to_str().unwrap());
            
            if file.path().is_dir() {
                unsafe {
                    FLUX.loaded_mapsets.push(BeatmapSet::from_folder(file.path().to_str().unwrap().to_string()));
                }
            } else {
                if file.path().extension().unwrap() == "sspm" {
                    SSPMParser::sspm_to_folder(file.path().to_str().unwrap());
                    unsafe {
                        FLUX.loaded_mapsets.push(BeatmapSet::from_folder(file.path().with_extension("").to_str().unwrap().to_string()));
                    }
                    continue
                }
            }
            

            
        }
    }
}