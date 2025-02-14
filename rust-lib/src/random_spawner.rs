use godot::classes::INode3D;
use godot::classes::Node3D;
use godot::classes::Node;
use godot::classes::PackedScene;
use godot::classes::RandomNumberGenerator;
use godot::obj::Base;
use godot::obj::Gd;
use godot::obj::NewGd;
use godot::obj::WithBaseField;
use godot::prelude::godot_api;
use godot::prelude::GodotClass;

use crate::random_util::get_rand_pos;

#[derive(GodotClass)]
#[class(base = Node3D, init)]
struct RandomSpawner{
    base: Base<Node3D>,
    #[export]
    min_spawns: i32,
    #[export]
    max_spawns: i32,
    #[export]
    other_point: Option<Gd<Node3D>>,
    #[export]
    spawn_scene: Option<Gd<PackedScene>>,
    #[export]
    target: Option<Gd<Node>>
}

#[godot_api]
impl INode3D for RandomSpawner{
    fn ready(&mut self){
        let mut random = RandomNumberGenerator::new_gd();
        let pos = self.base().get_global_position();
        let other_pos = self.get_other_point().map_or(pos, |v|v.get_position());
        let max = random.randi_range(self.min_spawns, self.max_spawns);
        for _count in 0..max {
            let mut spawn: Gd<Node3D> = self.spawn_scene
                .as_ref()
                .expect("no instantiation set")
                .instantiate()
                .expect("failed to init scene")
                .cast();
            let new_pos = get_rand_pos(&mut random, pos, other_pos);
            self.target.clone().unwrap().add_child(&spawn);
            spawn.set_global_position(new_pos);
        }
    }
    fn process(&mut self, _delta: f64){
        if self.base().get_child_count() == 0{
            self.base_mut().emit_signal("on_completed", &[]);
        }
    }
}