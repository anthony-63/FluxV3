use godot::prelude::*;

use crate::{content::maps::beatmap::NoteData, game::{cursor::Cursor, note::Note, Game}, FLUX};

use super::{note_renderer::NoteRenderer, sync_manager::SyncManager};

#[derive(GodotClass)]
#[class(base=Node)]
pub struct NoteManager {
    base: Base<Node>,
    game: Option<Gd<Game>>,
    sync_manager: Option<Gd<SyncManager>>,
    note_renderer: Option<Gd<NoteRenderer>>,
    hit_player: Option<Gd<AudioStreamPlayer>>,
    cursor: Option<Gd<Cursor>>,

    ordered_notes: Vec<Gd<Note>>,
    next_note: Option<Gd<Note>>,
    last_note: Option<Gd<Note>>,

    approach_time: f64,
    _skipped_notes: usize,
    pub notes_processing: usize,
    pub start_process: usize,

    pub colors: Vec<Color>,
    
    pushback: bool,
    started: bool,
}

#[godot_api]
impl INode for NoteManager {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            game: None,
            sync_manager: None,
            note_renderer: None,
            cursor: None,

            started: false,

            next_note: None,
            last_note: None,

            approach_time: 0.,
            notes_processing: 0,
            _skipped_notes: 0,
            start_process: 0,
            hit_player: None,
            pushback: false,

            ordered_notes: vec![],
            colors: vec![Color::from_html("#5d3fd3").unwrap(), Color::from_html("#ffe4ed").unwrap()]
        }
    }

    fn enter_tree(&mut self) {
        let game = self.base_mut().get_node_as::<Game>("../GameManager");
        let sync_manager = self.base().get_node_as::<SyncManager>("../SyncManager");
        let note_renderer = self.base().get_node_as::<NoteRenderer>("NoteRenderer");
        let cursor = self.base().get_node_as::<Cursor>("../Player/Cursor");
        let hit_player  = self.base().get_node_as::<AudioStreamPlayer>("../Hit");

        self.approach_time = unsafe { FLUX.settings.clone().unwrap().note.approach_time as f64 } * sync_manager.bind().speed as f64;

        self.pushback = unsafe { FLUX.settings.as_ref().unwrap().note.pushback };

        self.game = Some(game);
        self.sync_manager = Some(sync_manager);
        self.note_renderer = Some(note_renderer);
        self.cursor = Some(cursor);
        self.hit_player = Some(hit_player);
    }

    fn process(&mut self, _: f64) {
        if !self.started {
            return
        }

        let sync_manager = self.sync_manager.as_mut().unwrap().bind();
        let mut note_renderer = self.note_renderer.as_mut().unwrap().bind_mut();

        let mut to_render: Vec<Gd<Note>> = vec![];
        for i in self.start_process..self.ordered_notes.len() {
            let note = (&self.ordered_notes[i]).bind();
            if note.is_visible(sync_manager.real_time, sync_manager.speed, self.approach_time, self.pushback) {
                to_render.push(note.to_gd());
            }
            if note.time > sync_manager.real_time + self.approach_time { break; }
        }
        self.notes_processing = to_render.len();

        note_renderer.notes = to_render;
        note_renderer.update_instance_count();
    }

    fn physics_process(&mut self, _: f64,) {
        if !self.started {
            return
        }
        
        let sync_manager = self.sync_manager.as_mut().unwrap().bind();
        let cursor = self.cursor.as_mut().unwrap().bind();
        let mut game = self.game.as_mut().unwrap().bind_mut();

        let mut to_process: Vec<Gd<Note>> = vec![];
        for i in self.start_process..self.ordered_notes.len() {
            let note = (&self.ordered_notes[i]).bind();
            if note.calculate_time(sync_manager.real_time, self.approach_time) <= 0. && !note.hit {
                to_process.push(note.to_gd());
            }
            if note.time > sync_manager.real_time + self.approach_time {
                break;
            }
        }

        for mut note in to_process.into_iter() {
            let mut did_hitreg = false;
            let mut bound = note.bind_mut();

            if bound.is_touching(cursor.clamped_position) {
                bound.hit = true;
                did_hitreg = true;

                unsafe {
                    FLUX.score.as_mut().unwrap().hits += 1;
                    FLUX.score.as_mut().unwrap().total += 1;
                    FLUX.score.as_mut().unwrap().combo += 1;
                    FLUX.score.as_mut().unwrap().score += 25 * FLUX.score.as_ref().unwrap().multiplier;
                    FLUX.score.as_mut().unwrap().miniplier += 1;
                    if FLUX.score.as_mut().unwrap().miniplier >= 8 && FLUX.score.as_mut().unwrap().multiplier < 8 {
                        FLUX.score.as_mut().unwrap().miniplier = 0;
                        FLUX.score.as_mut().unwrap().multiplier = (FLUX.score.as_mut().unwrap().multiplier + 1).min(8);
                    }
                }

                game.health = (game.health + 1.25).min(10.);

                self.hit_player.as_mut().unwrap().play();
            }

            if !bound.hit && !bound.in_hit_window(sync_manager.real_time, sync_manager.speed) {
                did_hitreg = true;
                bound.hit = true;

                game.health = (game.health - 2.).max(0.);

                unsafe {
                    FLUX.score.as_mut().unwrap().misses += 1;
                    FLUX.score.as_mut().unwrap().total += 1;
                    FLUX.score.as_mut().unwrap().combo = 0;
                    FLUX.score.as_mut().unwrap().miniplier = 0;
                    FLUX.score.as_mut().unwrap().multiplier = (1_f32.max(FLUX.score.as_mut().unwrap().multiplier as f32 - 1.)) as usize;
                }
            }

            if did_hitreg {
                self.last_note = Some(bound.to_gd());
                self.next_note = Some(self.ordered_notes[bound.index].clone());
                self.start_process += 1;
            }
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

        for (i, note_data) in notes.into_iter().enumerate() {
            let mut note = Note::new_gd();

            note.bind_mut().time = note_data.time as f64;
            note.bind_mut().x = -note_data.x as f64;
            note.bind_mut().y = note_data.y as f64;
            note.bind_mut().index = i;
            note.bind_mut().color = *self.colors.get(i % self.colors.len()).unwrap();

            self.ordered_notes.push(note);
        }

        if self.ordered_notes.len() > 0 {
            self.next_note = Some(self.ordered_notes.get(0).unwrap().clone());
        }

        godot_print!("built {} notes", self.ordered_notes.len());
    }
}