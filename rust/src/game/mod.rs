use godot::prelude::*;

use self::cursor::Cursor;

pub mod cursor;

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct Game {
    base: Base<Node3D>,
    _cursor: Option<Gd<Cursor>>
}

#[godot_api]
impl INode3D for Game {
    fn init(base: Base<Node3D>) -> Self {
        Self {
            base,
            _cursor: None,
        }
    }

    fn enter_tree(&mut self) {
        let _cursor = self.base_mut().get_node_as::<Cursor>("Player/Cursor");
    }
}

#[godot_api]
impl Game {

}