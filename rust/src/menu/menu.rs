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
        let mut singleplayer = self.base().get_node_as::<Button>("Buttons/Singleplayer");
        let mut settings: Gd<Button> = self.base().get_node_as::<Button>("Buttons/Settings");

        singleplayer.connect("pressed".into(), self.base_mut().callable("singleplayer_pressed"));
        settings.connect("pressed".into(), self.base_mut().callable("settings_pressed"));
    }
}

#[godot_api]
impl Menu {
    #[func]
    fn singleplayer_pressed(&mut self) {
        self.base_mut().emit_signal("change_to_maplist".into(), &[]);
    }

    #[func]
    fn settings_pressed(&mut self) {
        self.base_mut().emit_signal("open_settings".into(), &[]);
    }

    #[signal]
    fn change_to_maplist();
    #[signal]
    fn open_settings();
}