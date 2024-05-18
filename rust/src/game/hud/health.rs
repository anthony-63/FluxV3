use godot::{engine::{IProgressBar, ProgressBar, Tween}, prelude::*};


#[derive(GodotClass)]
#[class(base=ProgressBar)]
pub struct HealthHUD {
    base: Base<ProgressBar>,
    tween: Option<Gd<Tween>>,
    hp: f32,
}

#[godot_api]
impl IProgressBar for HealthHUD {
    fn init(base: Base<ProgressBar>) -> Self {
        Self {
            base,
            tween: None,
            hp: 0.,
        }
    }

    fn enter_tree(&mut self) {
        let tween = self.base_mut().get_tree().unwrap().create_tween().unwrap();
        self.tween = Some(tween);
    }
}

#[godot_api]
impl HealthHUD {
    pub fn update(&mut self, health: f32) {
        if health == self.hp {
            return;
        }
        
        let mut tween = self.base_mut().get_tree().unwrap().create_tween().unwrap();

        tween.stop();
        tween.tween_property(self.base_mut().clone().upcast(), "value".into(), Variant::from(health), 0.1);
        self.hp = health;
        tween.play();
    }
}