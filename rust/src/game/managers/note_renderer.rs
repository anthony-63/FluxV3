use godot::{engine::{multi_mesh::TransformFormat, IMultiMeshInstance3D, MultiMeshInstance3D}, prelude::*};

use crate::{game::note::Note, FLUX};

use super::sync_manager::SyncManager;

#[derive(GodotClass)]
#[class(base=MultiMeshInstance3D)]
pub struct NoteRenderer {
    base: Base<MultiMeshInstance3D>,
    pub notes: Vec<Gd<Note>>,
    sync_manager: Option<Gd<SyncManager>>,
    approach_time: f64,
    approach_distance: f64,

    fade_in: f64,
    fade_out: f64,
}

#[godot_api]
impl IMultiMeshInstance3D for NoteRenderer {
    fn init(base: Base<MultiMeshInstance3D>) -> Self {
        Self {
            base,
            notes: vec![],
            approach_time: 0.,
            approach_distance: 0.,
            sync_manager: None,

            fade_in: 0.,
            fade_out: 0.,
        }
    }

    fn enter_tree(&mut self) {
        let sync_manager = self.base().get_node_as::<SyncManager>("../../SyncManager");

        self.approach_time = unsafe { FLUX.settings.as_ref().unwrap().note.approach_time as f64 };
        self.approach_distance = unsafe { FLUX.settings.as_ref().unwrap().note.approach_distance as f64 };

        let mut mesh = self.base_mut().get_multimesh().unwrap();
        mesh.set_instance_count(0);
        mesh.set_visible_instance_count(0);
        mesh.set_transform_format(TransformFormat::TRANSFORM_3D);
        self.base_mut().set_multimesh(mesh);

        self.fade_in = unsafe { FLUX.settings.as_ref().unwrap().note.fade_in as f64 } / 100.;

        if unsafe { FLUX.game.mods.ghost.enabled } {
            self.fade_out = unsafe { FLUX.game.mods.ghost.value as f64 } / 100.;
        } else if unsafe { FLUX.settings.as_ref().unwrap().note.half_ghost } {
            self.fade_out = 0.02;
        }else {
            self.fade_out = 0.;
        }

        self.sync_manager = Some(sync_manager);
    }

    fn process(&mut self, _: f64) {
        let sync_manager = self.sync_manager.as_ref().unwrap().bind();
        let mut mesh = self.base().get_multimesh().unwrap();

        for (i, note) in (&self.notes).into_iter().enumerate() {
            let bound = note.bind();

            let note_time = bound.calculate_time(sync_manager.real_time, self.approach_time * sync_manager.speed as f64);
            let note_distance = note_time * self.approach_distance;

            let mut fade_in = 1.;
            let mut fade_out = 1.;

            let ghost_amm = 1. - self.fade_out;
            
            let fade_in_start = self.approach_distance;
            let fade_in_end = self.approach_distance * (1. - self.fade_in);

            let fade_out_start = self.approach_distance * (1. - ghost_amm);
            let fade_out_end = self.approach_distance * (1. - ghost_amm + 0.24);

            let end_clamp;

            if unsafe { FLUX.game.mods.ghost.enabled } {
                end_clamp = 0.
            } else if unsafe { FLUX.settings.as_ref().unwrap().note.half_ghost } {
                end_clamp = 0.2;
            }else {
                end_clamp = 1.;
            }

            if self.fade_in != 0. {
                fade_in = Self::linstep(fade_in_start, fade_in_end, note_distance).powf(1.3 * 2.);
            }
            if self.fade_out != 0. {
                fade_out = Self::linstep(fade_out_start, fade_out_end, note_distance).powf(1.3 * 2.).max(end_clamp);
            }

            mesh.set_instance_transform(i as i32, Transform3D::new(Basis::IDENTITY, Vector3::new(bound.x as f32 * 2., bound.y as f32 * 2., -note_distance as f32)));
            mesh.set_instance_color(i as i32, Color::from_rgba(bound.color.r, bound.color.g, bound.color.b, fade_in.min(fade_out) as f32));
        }
        
        drop(sync_manager);
        self.base_mut().set_multimesh(mesh);
    }
}

impl NoteRenderer {
    pub fn linstep(a: f64, b: f64, x: f64) -> f64 {
        if a == b {
            return if x >= a {
                1.
            } else {
                0.
            }
        }
        return ((x - a) / (b - a)).clamp(0., 1.);
    }

    pub fn update_instance_count(&mut self) {
        let mut mesh = self.base_mut().get_multimesh().unwrap();
        if self.notes.len() > mesh.get_instance_count() as usize{
            mesh.set_instance_count(self.notes.len() as i32);
        }
        mesh.set_visible_instance_count(self.notes.len() as i32);
        self.base_mut().set_multimesh(mesh);
    }
}