use event_bus::EventBus;
use godot::classes::Engine;
use godot::global::godot_error;
use godot::init::gdextension; 
use godot::init::ExtensionLibrary;
use godot::init::InitLevel;
use godot::obj::NewAlloc;
struct RustExtension;

#[gdextension]
unsafe impl ExtensionLibrary for RustExtension {
    fn on_level_init(level: InitLevel) {
        if level == InitLevel::Scene {
            // The `&str` identifies your singleton and can be
            // used later to access it.
            Engine::singleton().register_singleton(
                "EventBus",
                &EventBus::new_alloc(),
            );
        }
    }

    fn on_level_deinit(level: InitLevel) {
        if level == InitLevel::Scene {
            // Let's keep a variable of our Engine singleton instance,
            // and MyEngineSingleton name.
            let mut engine = Engine::singleton();
            let singleton_name = "EventBus";

            // Here, we manually retrieve our singleton(s) that we've registered,
            // so we can unregister them and free them from memory - unregistering
            // singletons isn't handled automatically by the library.
            if let Some(my_singleton) = engine.get_singleton(singleton_name) {
                // Unregistering from Godot, and freeing from memory is required
                // to avoid memory leaks, warnings, and hot reloading problems.
                engine.unregister_singleton(singleton_name);
                my_singleton.free();
            } else {
                // You can either recover, or panic from here.
                godot_error!("Failed to get singleton");
            }
        }
    }
}

pub mod event_bus;
pub mod targets_counting;
pub mod load_scene;
pub mod call_to_delete;
pub mod random_spawner;
pub mod random_util;
pub mod random_init_velocity;
pub mod ui_display;
pub mod effects_on_free;
pub mod delete_after_ticks;
pub mod camera_tweener_3d;
pub mod third_person_character_body;
pub mod game_meters;