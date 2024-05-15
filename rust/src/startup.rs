use std::{sync::{Arc, Mutex}, thread};

use godot::{engine::{Label, Os, Time}, prelude::*};

use crate::{content::maploader::MapLoader, game::score::Score, settings::Settings, FLUX};

#[derive(GodotClass)]
#[class(base=Node)]
pub struct Startup {
    base: Base<Node>,
    internal: Arc<Mutex<StartupInternal>>,
    dot_timer: f64,
}

struct StartupInternal {
    stage: String,
}


#[godot_api]
impl INode for Startup {
    fn init(base: Base<Node>) -> Self {
        let startup = Self {
            base,
            internal: Arc::new(Mutex::new(StartupInternal{ stage: "Loading Flux".to_string() })),
            dot_timer: 0.,
        };

        return startup;
    }

    fn process(&mut self, delta: f64,) {
        self.dot_timer += delta;
        let mut label = self.base().get_node_as::<Label>("../Stage");

        let mut internal = self.internal.lock().unwrap();

        label.set_text(internal.stage.clone().into_godot());

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
            FLUX.score = Some(Score::default());
        }

        let internal = Arc::clone(&self.internal);
        thread::spawn(|| {
            Self::run_load(internal);
        });
    }
}

impl Startup { 
    fn run_load(internal: Arc<Mutex<StartupInternal>>) {
        let user_dir = Os::singleton().get_user_data_dir().to_string();

        internal.lock().unwrap().stage = "Loading settings".to_string();
        Self::load_settings();

        internal.lock().unwrap().stage = "Loading maps".to_string();
        Self::load_maps(user_dir.clone());
        internal.lock().unwrap().stage = "Done".to_string();
    }

    fn load_settings() {
        unsafe {
            FLUX.settings = Some(Settings::new());
            FLUX.settings.as_mut().unwrap().update(true);
        }
    }

    fn load_maps(user_dir: String) {
        let time = Time::singleton();

        let start = time.get_ticks_usec();
        MapLoader::load_all_from_dir(format!("{}/maps", user_dir));
        let end = time.get_ticks_usec();
        godot_print!("Loaded maps in {}ms", (end - start) / 1000);
    }
}