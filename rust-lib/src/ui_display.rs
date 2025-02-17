use godot::builtin::Callable;
use godot::builtin::GString;
use godot::classes::IProgressBar;
use godot::classes::ProgressBar;
use godot::obj::Base;
use godot::obj::WithBaseField;
use godot::prelude::godot_api;
use godot::prelude::GodotClass;

use crate::event_bus::EventBus;

#[derive(GodotClass)]
#[class(base = ProgressBar, init)]
struct StatDisplay{
    base: Base<ProgressBar>,
    #[export]
    stat_update_signal: GString
}

#[godot_api]
impl IProgressBar for StatDisplay{
    fn ready(&mut self){
        EventBus::singleton().connect(self.stat_update_signal.arg(), &Callable::from_object_method(&self.to_gd(), "update"));
    }
}

#[godot_api]
impl StatDisplay {
    #[func]
    fn update(&mut self, new_value: f64){
        self.base_mut().set_value(new_value);
    }
}