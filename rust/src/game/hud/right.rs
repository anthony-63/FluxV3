use godot::{engine::{Control, IControl, Label, ProgressBar, TextureProgressBar}, prelude::*};

use crate::FLUX;

#[derive(GodotClass)]
#[class(base=Control)]
pub struct RightHUD {
    base: Base<Control>,
    combo: Option<Gd<Label>>,
    misses: Option<Gd<Label>>,
    time_text: Option<Gd<Label>>,
    time_progress: Option<Gd<ProgressBar>>,
    score_multiplier: Option<Gd<TextureProgressBar>>,
    score_multiplier_text: Option<Gd<Label>>,
}

#[godot_api]
impl IControl for RightHUD {
    fn init(base: Base<Control>) -> Self {
        Self {
            base,
            combo: None,
            misses: None,
            time_text: None,
            time_progress: None,
            score_multiplier: None,
            score_multiplier_text: None,
        }
    }

    fn enter_tree(&mut self) {
        let combo = self.base().get_node_as::<Label>("Combo/Count");
        let misses = self.base().get_node_as::<Label>("Misses/Count");
        let time_text = self.base().get_node_as::<Label>("Timer");
        let time_progress = time_text.get_node_as::<ProgressBar>("Count");
        let score_multiplier = self.base().get_node_as::<TextureProgressBar>("Multiplier");
        let score_multiplier_text = score_multiplier.get_node_as::<Label>("Count");

        self.combo = Some(combo);
        self.misses = Some(misses);
        self.time_text = Some(time_text);
        self.time_progress = Some(time_progress);
        self.score_multiplier = Some(score_multiplier);
        self.score_multiplier_text = Some(score_multiplier_text);
    }
}

#[godot_api]
impl RightHUD {
    pub fn update(&mut self) {
        let score = unsafe { FLUX.score.as_ref().unwrap() };

        self.combo.as_mut().unwrap().set_text(format!("{}", score.combo).into());
        self.misses.as_mut().unwrap().set_text(format!("{}", score.misses).into());
        self.score_multiplier_text.as_mut().unwrap().set_text(format!("{}", score.multiplier).into());
        self.score_multiplier.as_mut().unwrap().set_value(score.miniplier as f64);
    }

    pub fn update_timer(&mut self, current: f32, length: f32) {
        self.time_text.as_mut().unwrap().set_text(format!("{:01}:{:02}/{:01}:{:02}",
                                                            (current / 60.).floor() as usize,
                                                            (current % 60.).floor() as usize,
                                                            (length / 60.).floor() as usize,
                                                            (length % 60.).floor() as usize).into());

        self.time_progress.as_mut().unwrap().set_max(length as f64);
        self.time_progress.as_mut().unwrap().set_value(current as f64);
    }
}