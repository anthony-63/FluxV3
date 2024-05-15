use std::path::Path;

use godot::{engine::{audio_stream_wav::LoopMode, AudioStream, AudioStreamMp3, AudioStreamWav}, prelude::*};

use super::{beatmap::Beatmap, sspm::AudioType};

#[derive(Default, Clone, GodotClass, Debug)]
#[class(base=RefCounted, no_init)]
pub struct BeatmapSet {
    pub broken: bool,

    pub version: u8,
    pub hash: String,
    pub path: String,

    pub artist: String,
    pub title: String,
    
    pub difficulties: Vec<Beatmap>,
    pub mappers: Vec<String>,

    pub music_path: String,
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
            difficulties.push(Beatmap::from_file(format!("{}/{}", folder_path, difficulty.to_string())));
        }

        let music_path = meta["_music"].to_string();
        
        Self {
            broken: false,
            version,
            title,
            mappers,
            music_path,
            difficulties,
            path: folder_path,
            ..Default::default()
        }
    }

    pub fn load_audio(&self, should_loop: bool) -> Option<Gd<AudioStream>> {
        let audio_bytes: Vec<u8> = match std::fs::read(format!("{}/music.bin", self.path)) {
            Ok(bytes) => bytes,
            Err(err) =>  {
                godot_error!("cant read audio from map: {}", err);
                return None;
            }
        };

        match AudioType::get_type(&audio_bytes) {
            AudioType::MP3 => {
                let mut stream = AudioStreamMp3::new_gd();
                stream.set_data(audio_bytes.as_slice().into());
                stream.set_loop(should_loop);
                return Some(stream.upcast());
            },
            AudioType::WAV => {
                let mut stream = AudioStreamWav::new_gd();
                stream.set_data(audio_bytes.as_slice().into());
                stream.set_loop_mode(if should_loop {
                    LoopMode::FORWARD
                } else {
                    LoopMode::DISABLED
                });
                return Some(stream.upcast());
            },
            _ => return None
        }
    }
}