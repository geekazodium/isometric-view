use core::f64;

use godot::classes::INode3D;
use godot::classes::Node3D;
use godot::obj::Base;
use godot::obj::Gd;
use godot::obj::WithBaseField;
use godot::prelude::godot_api;
use godot::prelude::GodotClass;


#[derive(GodotClass)]
#[class(base = Node3D)]
struct Tweener3D{
    base: Base<Node3D>,
    #[export] tween_velocity: f64,
    #[export] follow_target: Option<Gd<Node3D>>
}

#[godot_api]
impl INode3D for Tweener3D{
    fn init(base: Base<Node3D>) -> Self {
        Self{
            follow_target:None,
            tween_velocity:1.,
            base
        }
    }
    fn process(&mut self,delta: f64){
        if self.follow_target.is_none(){
            return;
        }
        let transform = self.follow_target.as_ref().unwrap().get_position();
        let this_transform = self.to_gd().get_position();
        let scale_fac = 1.0-f64::consts::E.powf(-delta * self.tween_velocity);
        let move_by = (transform - this_transform)*scale_fac as f32;
        let new_transform = self.base().get_transform().translated_local(move_by);
        self.base_mut().set_transform(new_transform);
    }
}