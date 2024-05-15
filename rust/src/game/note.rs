use godot::prelude::*;

use crate::content::maps::beatmap::NoteData;


#[derive(GodotClass)]
#[class(base=RefCounted)]
pub struct Note {
    base: Base<RefCounted>,
    pub data: Option<NoteData>,
}

#[godot_api]
impl IRefCounted for Note {
    fn init(base: Base<RefCounted>) -> Self {
        Self {
            base,
            data: None,
        }
    }
}

#[godot_api]
impl Note {

}