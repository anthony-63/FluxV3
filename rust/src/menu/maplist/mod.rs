use godot::{engine::{Button, Control, IControl}, prelude::*};

pub mod container;
pub mod map_button;

#[derive(GodotClass)]
#[class(base=Control)]
pub struct Maplist {
    base: Base<Control>,
}

#[godot_api]
impl IControl for Maplist {
    fn init(base: Base<Control>) -> Self {
        Self {
            base,
        }
    }

    fn enter_tree(&mut self,) {
        let mut button = self.base().get_node_as::<Button>("BottomBar/Back");
        button.connect("pressed".into(), self.base_mut().callable("back_pressed"));
    }
}

#[godot_api]
impl Maplist {
    #[func]
    fn back_pressed(&mut self) {
        self.base_mut().emit_signal("change_to_menu".into(), &[]);
    }

    #[signal]
    fn change_to_menu();
}