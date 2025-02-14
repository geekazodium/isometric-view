use godot::builtin::Vector3;
use godot::classes::INode;
use godot::classes::Node;
use godot::classes::RandomNumberGenerator;
use godot::classes::RigidBody3D;
use godot::obj::Base;
use godot::obj::NewGd;
use godot::obj::WithBaseField;
use godot::prelude::godot_api;
use godot::prelude::GodotClass;

use crate::random_util::get_rand_pos;

#[derive(GodotClass)]
#[class(base = Node, init)]
struct RandomInitVelocity{
    base: Base<Node>,
    #[export]
    min_vel: Vector3,
    #[export]
    max_vel: Vector3
}

#[godot_api]
impl INode for RandomInitVelocity {
    fn ready(&mut self){
        let mut random = RandomNumberGenerator::new_gd();
        let new_vel = get_rand_pos(
            &mut random, 
            self.min_vel,
            self.max_vel
        );
        self.base()
            .get_parent()
            .unwrap()
            .cast::<RigidBody3D>()
            .set_linear_velocity(new_vel);
    }
}