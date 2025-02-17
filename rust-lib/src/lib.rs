use godot::init::gdextension; 
use godot::init::ExtensionLibrary;

struct RustExtension;

#[gdextension]
unsafe impl ExtensionLibrary for RustExtension {}

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