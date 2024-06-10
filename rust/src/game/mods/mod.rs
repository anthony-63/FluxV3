use self::{ghost::GhostMod, nofail::NoFailMod, speed::SpeedMod};

pub mod speed;
pub mod nofail;
pub mod ghost;

#[derive(Clone, Default)]
pub struct AllMods {
    pub speed: SpeedMod,
    pub nofail: NoFailMod,
    pub ghost: GhostMod,
}