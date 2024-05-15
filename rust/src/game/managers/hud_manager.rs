use godot::prelude::*;

use crate::game::hud::left::LeftHUD;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct HudManager {
    base: Base<Node>,
    left: Option<Gd<LeftHUD>>,
}

#[godot_api]
impl INode for HudManager {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            left: None,
        }
    }

    fn enter_tree(&mut self) {
        let left_hud = self.base().get_node_as::<LeftHUD>("LeftViewport/HUD");
        
        self.left = Some(left_hud);
    }

    fn physics_process(&mut self, _: f64) {
        self.left.as_mut().unwrap().bind_mut().update();
    }
}

#[godot_api]
impl HudManager {
}