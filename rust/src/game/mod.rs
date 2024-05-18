use godot::{engine::{input::MouseMode, InputEvent}, prelude::*};

use crate::{content::maps::{beatmap::Beatmap, beatmapset::BeatmapSet}, FLUX};

use self::{cursor::Cursor, managers::{note_manager::NoteManager, sync_manager::SyncManager}, score::Score};

pub mod cursor;
pub mod managers;
pub mod debug;
pub mod note;
pub mod hud;
pub mod score;

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
            health: 10.,
            started_audio: false,
            started_notes: false,
        }
    }

    fn enter_tree(&mut self) {
        let mut sync_manager = self.base_mut().get_node_as::<SyncManager>("../SyncManager");
        let note_manager = self.base_mut().get_node_as::<NoteManager>("../NoteManager");

        let cursor = self.base_mut().get_node_as::<Cursor>("../Player/Cursor");

        unsafe { self.loaded_map = FLUX.selected_map.clone(); }
        unsafe { self.loaded_mapset = FLUX.selected_mapset.clone(); }

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
            self.end_game();
        }
    }

    fn process(&mut self, _: f64) {
        self.try_start();

        if self.health <= 0. {
            self.end_game();
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
            
            self.sync_manager.as_mut().unwrap().call("start".into(), &[(0.).to_variant()]);

            self.started_audio = true;
        }

        if !self.started_notes {
            self.note_manager.as_mut().unwrap().bind_mut().load_notes(self.loaded_map.as_ref().unwrap().bind().notes.clone());
            self.note_manager.as_mut().unwrap().call("start".into(), &[]);
            self.started_notes = true;
        }
    }

    fn end_game(&mut self) {
        Input::singleton().set_mouse_mode(MouseMode::VISIBLE);
        self.base_mut().get_tree().unwrap().change_scene_to_file("res://scenes/menu.tscn".into_godot());
    }
}