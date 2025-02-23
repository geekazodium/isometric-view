use godot::classes::Node;
use godot::obj::Base;
use godot::obj::WithBaseField;
use godot::prelude::godot_api;
use godot::prelude::GodotClass;

#[derive(GodotClass)]
#[class(base = Node, init)]
struct EntityDeleteOnDeath{
    base: Base<Node>
}

#[godot_api]
impl EntityDeleteOnDeath{
    #[func]
    fn on_health_change(&mut self, new_health: f64){
        if new_health > 0.{
            return;
        }
        self.base().get_parent().unwrap().queue_free();
    }
}