use godot::classes::INode;
use godot::classes::Node;
use godot::obj::Base;
use godot::obj::WithBaseField;
use godot::prelude::godot_api;
use godot::prelude::GodotClass;

#[derive(GodotClass)]
#[class(base = Node,init)]
pub struct DeleteAfterTicks {
    base: Base<Node>,
    delete_tick: u8,
    #[export]
    duration: u8,
    #[export]
    active: bool
}

#[godot_api]
impl INode for DeleteAfterTicks {
    fn physics_process(&mut self, _delta: f64) {
        if !self.active{
            return;
        }
        self.delete_tick += 1;
        if self.delete_tick > self.duration {
            self.base_mut().get_parent().unwrap().queue_free();
        }
    }
}