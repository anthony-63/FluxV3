use std::path::Path;

use godot::prelude::*;

use super::beatmap::Beatmap;

#[derive(Default)]
pub struct BeatmapSet {
    broken: bool,

    version: u8,
    hash: String,
    path: String,

    artist: String,
    title: String,
    
    difficulties: Vec<Beatmap>,
    mappers: Vec<String>,

    music_path: String,
}

impl BeatmapSet {
    pub fn from_folder(folder_path: String) -> Self {
        let meta_path = &format!("{}/meta.json", folder_path);
        if !Path::new(meta_path).exists() {
            godot_print!("meta.json not found for {}", folder_path);
            return Self {
                broken: true,
                ..Default::default()
            };
        }

        let meta_json = std::fs::read_to_string(meta_path).expect("meta json not found somehow?");
        let meta = json::parse(&meta_json).unwrap();

        let version = meta["_version"].as_u8().expect("version must be a number");
        let title = meta["_title"].to_string();

        let mut mappers: Vec<String> = vec![];
        for mapper in meta["_mappers"].members() {
            mappers.push(mapper.to_string());
        }

        let mut difficulties: Vec<Beatmap> = vec![];
        for difficulty in meta["_difficulties"].members() {
            difficulties.push(Beatmap::from_file(format!("{}/{}", folder_path, difficulty.to_string())))
        }

        let mut music_path = meta["_music"].to_string();
        
        Self {
            broken: false,
            version,
            title,
            mappers,
            music_path,
            ..Default::default()
        }
    }
}