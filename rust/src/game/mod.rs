use std::time::{SystemTime, UNIX_EPOCH};

use discord_rich_presence::activity::Timestamps;
use godot::{engine::{input::MouseMode, InputEvent}, prelude::*};

use crate::{content::maps::{beatmap::Beatmap, beatmapset::BeatmapSet}, flux::{flux_activity, set_activity}, FLUX};

use self::{cursor::Cursor, managers::{note_manager::NoteManager, sync_manager::SyncManager}, score::Score};

pub mod cursor;
pub mod managers;
pub mod debug;
pub mod note;
pub mod hud;
pub mod score;
pub mod mods;

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct Game {
    base: Base<Node3D>,
    loaded_map: Option<Gd<Beatmap>>,
    loaded_mapset: Option<Gd<BeatmapSet>>,
    sync_manager: Option<Gd<SyncManager>>,
    note_manager: Option<Gd<NoteManager>>,
    cursor: Option<Gd<Cursor>>,

    pub health: f32,
    
    started_audio: bool,
    started_notes: bool,
}

#[godot_api]
impl INode3D for Game {
    fn init(base: Base<Node3D>) -> Self {
        Self {
            base,
            cursor: None,
            loaded_map: None,
            loaded_mapset: None,
            sync_manager: None,
            note_manager: None,
            health: 6.,
            started_audio: false,
            started_notes: false,
        }
    }

    fn enter_tree(&mut self) {
        let mut sync_manager = self.base_mut().get_node_as::<SyncManager>("../SyncManager");
        let note_manager = self.base_mut().get_node_as::<NoteManager>("../NoteManager");

        unsafe { 
            self.loaded_map = FLUX.selected_map.clone();
            self.loaded_mapset = FLUX.selected_mapset.clone();
        }

        set_activity(flux_activity()
            .details(format!("Playing {}", self.loaded_mapset.as_ref().unwrap().bind().title).as_str())
            .timestamps(Timestamps::new()
                .start(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64 as i64)
                .end(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64 + self.loaded_map.as_ref().unwrap().bind().notes.last().unwrap().time as i64 - unsafe { FLUX.start_from } as i64)));

        sync_manager.connect("game_ended".into(), self.base_mut().callable("note_manager_ended"));

        let cursor = self.base_mut().get_node_as::<Cursor>("../Player/Cursor");

        let audio_stream = self.loaded_mapset.as_ref().unwrap().bind().load_audio(false).unwrap();

        unsafe {
            FLUX.score = Some(Score::default());
            FLUX.score.as_mut().unwrap().multiplier = 1;
        }
        
        sync_manager.call("set_stream".into(), &[audio_stream.to_variant()]);

        self.cursor = Some(cursor);
        self.sync_manager = Some(sync_manager);
        self.note_manager = Some(note_manager);
    }

    fn input(&mut self, _: Gd<InputEvent>) {
        if Input::singleton().is_action_just_pressed("end_map".into()) {
            self.fail_score();
        }
    }

    fn process(&mut self, _: f64) {
        self.try_start();

        if self.health <= 0. {
            self.fail_score();
        }
    }
}

#[godot_api]
impl Game {
    #[func]
    fn try_start(&mut self) {
        let sync_manager = self.sync_manager.as_ref().unwrap().bind();

        if !self.started_audio && sync_manager.start_timer > sync_manager.start_delay {
            drop(sync_manager);
            
            self.sync_manager.as_mut().unwrap().call("start".into(), &[(unsafe { FLUX.start_from - 1. }).to_variant()]);

            self.started_audio = true;
        }

        if !self.started_notes {            
            self.note_manager.as_mut().unwrap().bind_mut().load_notes(self.loaded_map.as_ref().unwrap().bind().notes.clone());
            self.note_manager.as_mut().unwrap().call("start".into(), &[]);
            self.started_notes = true;
        }
    }

    #[func]
    fn note_manager_ended(&mut self) {
        self.end_game();
    }

    fn fail_score(&mut self) {
        unsafe { 
            FLUX.score.as_mut().unwrap().failed = true; 
            FLUX.score.as_mut().unwrap().fail_time = self.sync_manager.as_ref().unwrap().bind().real_time;
        }
        self.end_game();
    }

    fn end_game(&mut self) {
        Input::singleton().set_mouse_mode(MouseMode::VISIBLE);
        unsafe { 
            FLUX.should_open_details = true;
            FLUX.score.as_mut().unwrap().mods_used = FLUX.mods.clone();
            FLUX.score.as_mut().unwrap().map_id = FLUX.selected_mapset.as_ref().unwrap().bind().hash.clone() + "/" + &FLUX.selected_map.as_ref().unwrap().bind().name;
        };
        self.base_mut().get_tree().unwrap().change_scene_to_file("res://scenes/menu.tscn".into_godot());
    }
}