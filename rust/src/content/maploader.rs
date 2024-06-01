use std::{path::{Path, PathBuf}, sync::{Arc, Mutex}};

use godot::{engine::Os, prelude::*};

use crate::{content::maps::{beatmapset::BeatmapSet, sspm::SSPMParser}, startup::StartupInternal, FLUX};

pub struct MapLoader;

impl MapLoader {
    pub fn load_all_from_dir(path: String, update_label: fn(String, usize, usize, Arc<Mutex<StartupInternal>>), internal: Arc<Mutex<StartupInternal>>) {
        godot_print!("loading all maps from {}", path);

        let map_count = std::fs::read_dir(path.clone()).unwrap().count();
        let map_folders = std::fs::read_dir(path).unwrap();

        for (index, filename) in map_folders.into_iter().enumerate() {
            let file: std::fs::DirEntry = filename.unwrap();

            if file.path().is_dir() {
                unsafe {
                    FLUX.maps.loaded_mapsets.push(BeatmapSet::from_folder(file.path().to_str().unwrap().to_string()));
                }
                update_label(file.file_name().to_string_lossy().to_string(), index, map_count, internal.clone());
            } else {
                godot_print!("{}", file.path().to_string_lossy());
                if file.path().to_string_lossy().ends_with(".sspm") {
                    let parsed = SSPMParser::sspm_to_folder(file.path().to_str().unwrap(), true);

                    if parsed {
                        unsafe {
                            FLUX.maps.loaded_mapsets.push(BeatmapSet::from_folder(file.path().with_extension("").to_str().unwrap().to_string()));
                        }
                    }
                    update_label(file.file_name().to_string_lossy().to_string(), index, map_count, internal.clone());
                    continue
                }
            }
        }
    }

    pub fn add_map(path: String) {
        godot_print!("{}", path);
        if path.ends_with(".sspm") {
            let user_dir = Os::singleton().get_user_data_dir().to_string();
            let folder_name = PathBuf::from(path.clone()).with_extension("").file_name().unwrap().to_str().unwrap().to_string();
    
            let folder_path = format!("{}/{}", user_dir, folder_name);
    
            SSPMParser::sspm_to_folder(&path, false);
            
            unsafe {
                FLUX.maps.loaded_mapsets.push(BeatmapSet::from_folder(Path::new(&folder_path).with_extension("").to_str().unwrap().to_string()));
            }
        }
    }
}