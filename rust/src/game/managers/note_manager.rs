use godot::prelude::*;

use crate::{content::maps::beatmap::NoteData, game::{note::Note, Game}, FLUX};

use super::sync_manager::SyncManager;

static HIT_WINDOW: f64 = 0.050;
static AABB: f64 = (1.75 + 0.525) / 2.;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct NoteManager {
    base: Base<Node>,
    game: Option<Gd<Game>>,
    sync_manager: Option<Gd<SyncManager>>,

    ordered_notes: Vec<Gd<Note>>,
    next_note: Option<Gd<Note>>,
    last_note: Option<Gd<Note>>,

    approach_time: f32,
    skipped_notes: usize,
    notes_processing: usize,
    start_process: usize,

    started: bool,
}

#[godot_api]
impl INode for NoteManager {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            game: None,
            sync_manager: None,
            started: false,

            approach_time: 0.,
            last_note: None,
            next_note: None,
            notes_processing: 0,
            ordered_notes: vec![],
            skipped_notes: 0,
            start_process: 0,
        }
    }

    fn enter_tree(&mut self) {
        let game = self.base_mut().get_node_as::<Game>("../GameManager");
        let sync_manager = self.base_mut().get_node_as::<SyncManager>("../SyncManager");

        self.approach_time = unsafe { FLUX.settings.clone().unwrap().note.approach_time };

        self.game = Some(game);
        self.sync_manager = Some(sync_manager);
    }

    fn process(&mut self, _: f64) {
        if !self.started {
            return
        }
    }

    fn physics_process(&mut self, _: f64,) {
        if !self.started {
            return
        }
    }
}

#[godot_api]
impl NoteManager {
    #[func]
    pub fn start(&mut self) {
        self.started = true;
    }
}

impl NoteManager {
    pub fn load_notes(&mut self, notes_array: Vec<NoteData>) {
        let mut notes = notes_array.clone();
        notes.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());

        for note_data in notes {
            let mut note = Note::new_gd();

            note.bind_mut().data = Some(note_data);

            self.ordered_notes.push(note);
        }

        if self.ordered_notes.len() > 0 {
            self.next_note = Some(self.ordered_notes.get(0).unwrap().clone());
        }

        godot_print!("built {} notes", self.ordered_notes.len());
    }
}