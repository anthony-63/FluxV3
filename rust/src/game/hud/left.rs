use std::ops::Div;

use godot::{engine::{Control, IControl, Label}, prelude::*};

use crate::FLUX;

#[derive(GodotClass)]
#[class(base=Control)]
pub struct LeftHUD {
    base: Base<Control>,
    accuracy: Option<Gd<Label>>
}

#[godot_api]
impl IControl for LeftHUD {
    fn init(base: Base<Control>) -> Self {
        Self {
            base,
            accuracy: None,
        }
    }

    fn enter_tree(&mut self) {
        let accuracy = self.base().get_node_as::<Label>("Accuracy");

        self.accuracy = Some(accuracy);
    }
}

#[godot_api]
impl LeftHUD {
    pub fn update(&mut self) {
        let acc_label = self.accuracy.as_mut().unwrap();
        let score = unsafe { FLUX.score.as_ref().unwrap() };

        let acc = (score.hits as f64).div(score.total as f64) * 100.;
        acc_label.set_text(format!("{:.2}%", acc).into())
    }
}