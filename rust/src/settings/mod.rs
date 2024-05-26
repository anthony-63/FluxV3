use godot::{engine::AudioServer, prelude::*};
use serde::{Deserialize, Serialize};

pub mod loader;
pub mod saver;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ApproachMode {
    DistTime,
    DistRate,
    RateTime,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NoteSettings {
    pub approach_rate: f32,
    pub approach_time: f32,
    pub approach_distance: f32,
    pub pushback: bool,
    pub approach_mode: ApproachMode,
    pub fade_in: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AudioSettings {
    pub master_volume: f32,
    pub music_volume: f32,
    pub sfx_volume: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CursorSettings {
    pub sensitivity: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CameraSettings {
    pub spin: bool,
    pub parallax: f32,
}

#[derive(GodotClass, Clone, Debug, Serialize, Deserialize)]
#[class(base=RefCounted, no_init)]
pub struct Settings {
    pub note: NoteSettings,
    pub audio: AudioSettings,
    pub cursor: CursorSettings,
    pub camera: CameraSettings,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            note: NoteSettings {
                approach_rate: 50.,
                approach_time: 1.,
                approach_distance: 50.,
                approach_mode: ApproachMode::RateTime,
                pushback: true,
                fade_in: 50.,
            },
            audio: AudioSettings {
                master_volume: 0.5,
                music_volume: 0.5,
                sfx_volume: 0.5,
            },
            cursor: CursorSettings {
                sensitivity: 0.5,
            },
            camera: CameraSettings {
                spin: false,
                parallax: 10.,
            },
        }
    }

    pub fn update(&mut self, loading: bool) {
        if loading {
            self.load("settings.bin".into());
        } else {
            self.save("settings.bin".into());
        }

        match self.note.approach_mode {
            ApproachMode::DistTime => self.note.approach_rate = self.note.approach_distance / self.note.approach_time,
            ApproachMode::RateTime =>  self.note.approach_distance = self.note.approach_rate * self.note.approach_time,
            ApproachMode::DistRate => self.note.approach_time = self.note.approach_distance / self.note.approach_rate,
        }

        let mut audio_server = AudioServer::singleton();
        
        let master_bus = audio_server.get_bus_index("Master".into());
        audio_server.set_bus_volume_db(master_bus, 10. * self.audio.master_volume.log10());

        let music_bus = audio_server.get_bus_index("Music".into());
        audio_server.set_bus_volume_db(music_bus, 10. * self.audio.music_volume.log10());

        let sfx_bus = audio_server.get_bus_index("SFX".into());
        audio_server.set_bus_volume_db(sfx_bus, 10. * self.audio.sfx_volume.log10());
    }
}