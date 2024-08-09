use std::{sync::{Arc, Mutex}, thread};

use discord_rich_presence::{DiscordIpc, DiscordIpcClient};
use godot::{engine::{Label, Os, ProgressBar, Time}, prelude::*};
use rand::prelude::SliceRandom;
use crate::{content::maploader::MapLoader, flux::{flux_activity, set_activity}, game::score::Score, settings::Settings, FLUX};

#[derive(GodotClass)]
#[class(base=Node)]
pub struct Startup {
    base: Base<Node>,
    internal: Arc<Mutex<StartupInternal>>,
    dot_timer: f64,
}

pub struct StartupInternal {
    stage: String,
    substatus: String,
    progress_count: usize,
    progress_total: usize,
    has_progress: bool,
}


#[godot_api]
impl INode for Startup {
    fn init(base: Base<Node>) -> Self {
        let startup = Self {
            base,
            internal: Arc::new(Mutex::new(StartupInternal{ 
                stage: "Loading Flux".to_string(),
                substatus: "".to_string(), 
                progress_count: 0, 
                progress_total: 0,
                has_progress: false,
            })),
            dot_timer: 0.,
        };

        return startup;
    }

    fn process(&mut self, delta: f64,) {
        self.dot_timer += delta;
        let mut label = self.base().get_node_as::<Label>("../Stage");
        let mut substatus = self.base().get_node_as::<Label>("../SubStatus");
        let mut progress = self.base().get_node_as::<ProgressBar>("../Progress");

        let mut internal = self.internal.lock().unwrap();

        progress.set_max(internal.progress_total as f64);
        progress.set_value(internal.progress_count as f64);
        progress.set_visible(internal.has_progress);
        label.set_text(internal.stage.clone().into_godot());
        substatus.set_text(internal.substatus.clone().into_godot());

        if self.dot_timer > 0.2 {
            if internal.stage.contains("...") {
                internal.stage = internal.stage.replace("...", "");
            }
            else {
                internal.stage = format!("{}.", internal.stage);
            }

            self.dot_timer = 0.
        }

        if internal.stage.contains("Done") {
            drop(internal);
            self.base_mut().get_tree().unwrap().change_scene_to_file("res://scenes/menu.tscn".into_godot());
        }
    }

    fn enter_tree(&mut self) {
        unsafe {
            FLUX.game.score = Some(Score::default());
        }

        let internal = Arc::clone(&self.internal);
        thread::spawn(|| {
            Self::run_load(internal);
        });
    }
}

impl Startup { 
    fn run_load(internal: Arc<Mutex<StartupInternal>>) {
        unsafe { 
            FLUX.discord_client = Some(DiscordIpcClient::new("1231849122336604171").unwrap());
            match FLUX.discord_client.as_mut().unwrap().connect() {
                Ok(_) => {},
                Err(e) => {
                    godot_print!("failed to setup rpc with error {}", e);
                }
            }
        }


        set_activity(flux_activity().details("Starting Up"));

        let user_dir = Os::singleton().get_user_data_dir().to_string();
        godot_print!("loading maps from {}/maps", user_dir.clone());

        internal.lock().unwrap().stage = "Loading settings".to_string();
        Self::load_settings();

        internal.lock().unwrap().stage = "Loading maps".to_string();
        internal.lock().unwrap().has_progress = true;
        fn update_substatus(name: String, count: usize, total: usize, internal: Arc<Mutex<StartupInternal>>) {
            internal.lock().unwrap().substatus = name;
            internal.lock().unwrap().progress_count = count;
            internal.lock().unwrap().progress_total = total;
        }
        Self::load_maps(user_dir.clone(), update_substatus, internal.clone());

        internal.lock().unwrap().stage = "Selecting map...".to_string();
        internal.lock().unwrap().has_progress = false;
        unsafe {
            if FLUX.maps.loaded_mapsets.len() > 0 {
                FLUX.game.selected_mapset = Some(Gd::from_object(FLUX.maps.loaded_mapsets.choose(&mut rand::thread_rng()).unwrap().clone()));
                FLUX.game.selected_map = Some(Gd::from_object(FLUX.game.selected_mapset.clone().unwrap().bind().difficulties.choose(&mut rand::thread_rng()).unwrap().clone()));
            }
        }

        internal.lock().unwrap().stage = "Done".to_string();
    }

    fn load_settings() {
        unsafe {
            FLUX.settings = Some(Settings::new());
            FLUX.settings.as_mut().unwrap().update(true);
        }
    }

    fn load_maps(user_dir: String, update_label: fn(String, usize, usize, Arc<Mutex<StartupInternal>>), internal: Arc<Mutex<StartupInternal>>) {
        let time = Time::singleton();

        let start = time.get_ticks_usec();

        MapLoader::load_all_from_dir(format!("{}/maps", user_dir), update_label, internal);
        let end = time.get_ticks_usec();
        godot_print!("Loaded maps in {}ms", (end - start) / 1000);
    }
}