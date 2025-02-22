use godot::builtin::GString;
use godot::meta::ToGodot;
use godot::obj::Base; 
use godot::obj::WithBaseField;
use godot::prelude::godot_api;
use godot::prelude::GodotClass;
use godot::classes::INode;
use godot::classes::Node;

use crate::event_bus::EventBus;

#[derive(GodotClass)]
#[class(base = Node, init)]
pub struct TickingStatTracker{
    base: Base<Node>,
    #[export] max_meter: f64,
    #[export] current_meter: f64,
    #[export] increase_per_second: f64,
    #[export] update_event_name: GString
}

#[godot_api]
impl INode for TickingStatTracker{
    fn ready(&mut self){
        self.emit_update_signal();
    }
    fn physics_process(&mut self, delta: f64){
        self.current_meter += delta * self.increase_per_second;
        self.clamp_stat();
        if delta * self.increase_per_second == 0.{
            return;
        }
        self.emit_update_signal();
    }
}

#[godot_api]
impl TickingStatTracker{
    #[signal]
    fn meter_changed(current: f64);
    #[func]
    pub fn add_to_stat(&mut self, amount: f64){
        self.current_meter += amount;
        self.clamp_stat();
    }
    fn clamp_stat(&mut self){
        self.current_meter = self.current_meter.clamp(0., self.max_meter);
    }
}

impl TickingStatTracker{
    fn emit_update_signal(&mut self){
        let curr = self.current_meter.to_variant();
        if !self.update_event_name.is_empty(){
            // order of operations should be from local change then handle global event
            self.base_mut().emit_signal("meter_changed", &[curr.clone()]);
            EventBus::singleton().emit_signal(self.update_event_name.arg(), &[curr]);
        }else{
            self.base_mut().emit_signal("meter_changed", &[curr]);
        }
    }
}