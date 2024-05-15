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
        }
    }

    fn enter_tree(&mut self) {
        let sync_manager = self.base().get_node_as::<SyncManager>("../../SyncManager");

        self.approach_time = unsafe { FLUX.settings.clone().unwrap().note.approach_time as f64 };
        self.approach_distance = unsafe { FLUX.settings.clone().unwrap().note.approach_distance as f64 };

        let mut mesh = self.base_mut().get_multimesh().unwrap();
        mesh.set_instance_count(0);
        mesh.set_visible_instance_count(0);
        mesh.set_transform_format(TransformFormat::TRANSFORM_3D);
        self.base_mut().set_multimesh(mesh);

        self.sync_manager = Some(sync_manager);
    }

    fn process(&mut self, _: f64) {
        let sync_manager = self.sync_manager.as_ref().unwrap().bind();
        let mut mesh = self.base().get_multimesh().unwrap();

        for (i, note) in (&self.notes).into_iter().enumerate() {
            let bound = note.bind();

            let note_time = bound.calculate_time(sync_manager.real_time, self.approach_time);
            let note_distance = note_time * self.approach_distance;
            mesh.set_instance_transform(i as i32, Transform3D::new(Basis::IDENTITY, Vector3::new(bound.x as f32 * 2., bound.y as f32 * 2., -note_distance as f32)));
            mesh.set_instance_color(i as i32, bound.color);
        }

        drop(sync_manager);
        self.base_mut().set_multimesh(mesh.clone());
    }
}

impl NoteRenderer {
    pub fn update_instance_count(&mut self) {
        let mut mesh = self.base_mut().get_multimesh().unwrap();
        if self.notes.len() > mesh.get_instance_count() as usize{
            mesh.set_instance_count(self.notes.len() as i32);
        }
        mesh.set_visible_instance_count(self.notes.len() as i32);
        self.base_mut().set_multimesh(mesh);
    }
}