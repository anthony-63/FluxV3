use godot::{engine::Engine, prelude::*};

pub mod flux;
pub mod startup;

struct MyExtension;


#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {
    // fn on_level_init(level: InitLevel) {
    //     if level == InitLevel::Scene {
    //         Engine::singleton().register_singleton(
    //             StringName::from("Flux"),
    //             flux::Flux::new_alloc().upcast(),
    //         );
    //     }
    // }

    // fn on_level_deinit(level: InitLevel) {
    //     if level == InitLevel::Scene {
    //         let mut engine = Engine::singleton();
    //         let singleton_name = StringName::from("Flux");

    //         let singleton = engine
    //             .get_singleton(singleton_name.clone())
    //             .expect("cannot retrieve the singleton");

    //         engine.unregister_singleton(singleton_name);
    //         singleton.free();
    //     }
    // }
}