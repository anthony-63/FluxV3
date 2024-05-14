use godot::{engine::{Button, Control, IControl}, prelude::*};

#[derive(GodotClass)]
#[class(base=Control)]
pub struct Menu {
    base: Base<Control>,
}

#[godot_api]
impl IControl for Menu {
    fn init(base: Base<Control>) -> Self {
        Self {
            base,
        }
    }

    fn enter_tree(&mut self,) {
        let mut button = self.base().get_node_as::<Button>("Buttons/Singleplayer");
        button.connect("pressed".into(), self.base_mut().callable("singleplayer_pressed"));
    }
}

#[godot_api]
impl Menu {
    #[func]
    fn singleplayer_pressed(&mut self) {
        self.base_mut().emit_signal("change_to_maplist".into(), &[]);
    }

    #[signal]
    fn change_to_maplist();
}