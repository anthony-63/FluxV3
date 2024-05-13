use std::borrow::{Borrow, BorrowMut};

use godot::{engine::{Control, IControl, Label}, prelude::*};

#[derive(GodotClass)]
#[class(base=Node)]
pub struct Startup {
    base: Base<Node>
}

#[godot_api]
impl INode for Startup {
    fn init(base: Base<Node>) -> Self {
        let startup = Self {
            base,
        };

        return startup;
    }

    fn enter_tree(&mut self) {
        godot_print!("hi");
        let mut stage_label = self.base().get_node_as::<Label>("../Stage");
        stage_label.set_text("HI!".to_godot());
    }
}