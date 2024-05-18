use godot::prelude::*;

use crate::game::{hud::{health::HealthHUD, left::LeftHUD}, Game};

#[derive(GodotClass)]
#[class(base=Node)]
pub struct HudManager {
    base: Base<Node>,
    left: Option<Gd<LeftHUD>>,
    health: Option<Gd<HealthHUD>>,
    game: Option<Gd<Game>>,
}

#[godot_api]
impl INode for HudManager {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            left: None,
            health: None,
            game: None,
        }
    }

    fn enter_tree(&mut self) {
        let left_hud = self.base().get_node_as::<LeftHUD>("LeftViewport/HUD");
        let health_hud = self.base().get_node_as::<HealthHUD>("HealthViewport/Health");
        let game = self.base().get_node_as::<Game>("../GameManager");

        self.left = Some(left_hud);
        self.health = Some(health_hud);
        self.game = Some(game);
    }

    fn physics_process(&mut self, _: f64) {
        let game = self.game.as_ref().unwrap().bind();

        self.left.as_mut().unwrap().bind_mut().update();
        self.health.as_mut().unwrap().bind_mut().update(game.health);
    }
}

#[godot_api]
impl HudManager {
}