use std::{borrow::{Borrow, BorrowMut}, sync::{mpsc, Arc, Mutex}, thread};

use godot::{engine::{os, Control, DirAccess, DirectionalLight2D, IControl, Label, Os}, prelude::*};

use crate::content::maploader::MapLoader;

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
    }

    fn enter_tree(&mut self) {        
        let internal = Arc::clone(&self.internal);
        
        thread::scope(|s| {
            s.spawn(|| {
                Self::run_load(internal);
            });
        });
    }

    
}

impl Startup { 
    fn run_load(internal: Arc<Mutex<StartupInternal>>) {
        internal.lock().unwrap().stage = "Loading maps".to_string();
    }
}