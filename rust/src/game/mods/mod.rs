use self::{nofail::NoFailMod, speed::SpeedMod};

pub mod speed;
pub mod nofail;

#[derive(Clone, Default)]
pub struct AllMods {
    pub speed: SpeedMod,
    pub nofail: NoFailMod,
}