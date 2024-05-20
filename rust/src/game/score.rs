use core::f64;
use std::ops::Div;

use super::mods::AllMods;

#[derive(Clone, Default)]
pub struct Score {
    pub map_id: String,
    pub failed: bool,
    pub fail_time: f64,
    pub hits: usize,
    pub misses: usize,
    pub total: usize,
    pub score: usize,
    pub combo: usize,
    pub max_combo: usize,
    pub multiplier: usize,
    pub miniplier: usize,
    pub mods_used: AllMods,
}

impl Score {
    pub fn get_accuracy(&self) -> f64 {
        return (self.hits as f64).div(self.total as f64) * 100.;
    }

    pub fn get_rank(&self) -> String {
        let acc = self.get_accuracy();

        match acc {
            99_f64..=f64::MAX => return "SS".into(),
            95_f64..=f64::MAX => return "S".into(),
            90_f64..=f64::MAX => return "A".into(),
            85_f64..=f64::MAX => return "B".into(),
            75_f64..=f64::MAX => return "C".into(),
            65_f64..=f64::MAX => return "D".into(),
            _ => return "F".into(),
        }
    }
}