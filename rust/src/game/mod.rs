use godot::prelude::*;

use crate::{content::maps::{beatmap::Beatmap, beatmapset::BeatmapSet}, FLUX};

use self::{cursor::Cursor, managers::sync_manager::SyncManager};

pub mod cursor;
pub mod managers;

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct Game {
    base: Base<Node3D>,
    loaded_map: Option<Gd<Beatmap>>,
    loaded_mapset: Option<Gd<BeatmapSet>>,
    sync_manager: Option<Gd<SyncManager>>,
    cursor: Option<Gd<Cursor>>,
    started: bool,
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
            started: false,
        }
    }

    fn enter_tree(&mut self) {
        let mut sync_manager = self.base_mut().get_node_as::<SyncManager>("../SyncManager");
        let cursor = self.base_mut().get_node_as::<Cursor>("../Player/Cursor");

        unsafe { self.loaded_map = FLUX.selected_map.clone(); }
        unsafe { self.loaded_mapset = FLUX.selected_mapset.clone(); }

        godot_print!("{}", self.loaded_map.as_ref().unwrap().bind().name);

        let audio_stream = self.loaded_mapset.as_ref().unwrap().bind().load_audio(false).unwrap();

        sync_manager.call("set_stream".into(), &[audio_stream.to_variant()]);


        self.cursor = Some(cursor);
        self.sync_manager = Some(sync_manager);
    }

    fn process(&mut self, _: f64) {
        self.try_start();
    }
    
}

#[godot_api]
impl Game {
    #[func]
    fn try_start(&mut self) {
        if !self.started {
            self.sync_manager.as_mut().unwrap().call("start".into(), &[(0.).to_variant()]);
            self.started = true;
        }
    }
}