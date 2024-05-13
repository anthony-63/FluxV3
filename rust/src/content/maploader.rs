use godot::prelude::*;


pub struct MapLoader;
impl MapLoader {
    pub fn load_all_from_dir(path: String) {
        godot_print!("loading all maps from {}", path);

        let map_folders = std::fs::read_dir(path).unwrap();

        for folder in map_folders {
            godot_print!("loading map {}", folder.unwrap().path().display());
        }
    }
}