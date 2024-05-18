use godot::{engine::{Control, IControl, Label}, prelude::*};

use crate::FLUX;

#[derive(GodotClass)]
#[class(base=Control)]
pub struct LeftHUD {
    base: Base<Control>,
    accuracy: Option<Gd<Label>>,
    rank: Option<Gd<Label>>,
    score: Option<Gd<Label>>,
}

#[godot_api]
impl IControl for LeftHUD {
    fn init(base: Base<Control>) -> Self {
        Self {
            base,
            accuracy: None,
            rank: None,
            score: None,
        }
    }

    fn enter_tree(&mut self) {
        let accuracy = self.base().get_node_as::<Label>("Accuracy");
        let score = self.base().get_node_as::<Label>("Score/Count");
        let rank = accuracy.get_node_as::<Label>("Rank");

        self.accuracy = Some(accuracy);
        self.score = Some(score);
        self.rank = Some(rank);
    }
}

#[godot_api]
impl LeftHUD {
    pub fn update(&mut self) {
        let score = unsafe { FLUX.score.as_ref().unwrap() };

        self.accuracy.as_mut().unwrap().set_text(format!("{:.2}%", score.get_accuracy()).into());
        self.rank.as_mut().unwrap().set_text(score.get_rank().into());
        self.score.as_mut().unwrap().set_text(format!("{}", score.score).into());
    }
}