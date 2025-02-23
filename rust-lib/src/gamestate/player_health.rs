use godot::classes::Node;
use godot::classes::Node3D;
use godot::obj::Base;
use godot::obj::WithBaseField;
use godot::prelude::godot_api;
use godot::prelude::GodotClass;

#[derive(GodotClass)]
#[class(base = Node, init)]
struct PlayerDisableOnDeath{
    base: Base<Node>
}

#[godot_api]
impl PlayerDisableOnDeath{
    #[func]
    fn on_health_change(&mut self, new_health: f64){
        if new_health > 0.{
            return;
        }
        let mut parent = self.base().get_parent().unwrap();
        parent.set_process(false);
        parent.set_physics_process(false);
        parent.cast::<Node3D>().set_visible(false);
    }
}