use self::speed::SpeedMod;

pub mod speed;

#[derive(Clone, Default)]
pub struct AllMods {
    pub speed: SpeedMod,
}