use godot::{engine::{EditorPlugin, IEditorPlugin}, prelude::*};

#[derive(GodotClass)]
#[class(init, base=Object)]
pub struct Flux {
    base: Base<Object>,
}

#[godot_api]
impl Flux {
    
}