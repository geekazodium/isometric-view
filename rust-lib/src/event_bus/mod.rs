use godot::builtin::StringName;
use godot::classes::Object;
use godot::obj::Base;
use godot::obj::Gd;
use godot::prelude::godot_api;
use godot::prelude::GodotClass;

#[derive(GodotClass)]
#[class(base = Object, init)]
pub struct EventBus{
    base: Base<Object>
}

#[godot_api]
impl EventBus{
    #[signal]
    fn player_health_changed(new_amount: f64);
    #[signal]
    fn player_sanity_changed(new_amount: f64);
}

impl EventBus{
    pub fn singleton() -> Gd<Self>{
        godot::classes::Engine::singleton()
            .get_singleton(&StringName::from("EventBus"))
            .expect("Event Bus Not Registered!!!")
            .cast()
    }
}