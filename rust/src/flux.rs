use discord_rich_presence::{activity::{self, Activity, Assets}, DiscordIpc, DiscordIpcClient};
use godot::{engine::ImageTexture, log::godot_warn, obj::Gd};

use crate::{content::maps::{beatmap::Beatmap, beatmapset::BeatmapSet}, game::{mods::AllMods, score::Score}, settings::Settings, FLUX};

pub struct Flux {
    pub loaded_mapsets: Vec<BeatmapSet>,

    pub total_diff_count: usize,

    pub settings: Option<Settings>,
    pub score: Option<Score>,

    pub fullscreen: bool,

    pub selected_mapset: Option<Gd<BeatmapSet>>,
    pub selected_map: Option<Gd<Beatmap>>,

    pub start_from: f64,

    pub should_open_details: bool,

    pub mods: AllMods,

    pub covers_instance_holder: Vec<Gd<ImageTexture>>,

    pub discord_client: Option<DiscordIpcClient>,
}

pub fn flux_activity() -> Activity<'static> {
    let mut activity = activity::Activity::new();
    activity = activity.assets(Assets::new().large_image("fluxlogo"));
    activity
}

pub fn set_activity(activity: Activity) {
    unsafe {
        match FLUX.discord_client.as_mut().unwrap().set_activity(activity) {
            Ok(_) => {},
            Err(e) => {
                godot_warn!("failed to set discord activity with error {}", e);
            }
        }
    }
}