use godot::classes::CharacterBody3D;
use godot::classes::IRayCast3D;
use godot::classes::MeshInstance3D;
use godot::classes::RayCast3D;
use godot::obj::Base;
use godot::obj::Gd;
use godot::obj::WithBaseField;
use godot::prelude::godot_api;
use godot::prelude::GodotClass;
use godot::prelude::Vector3;

use crate::game_meters::ticking_stat_tracker::TickingStatTracker;

const HEALTH_TRACKER_NAME: &str = "HealthTracker";

#[derive(GodotClass)]
#[class(base = RayCast3D, init)]
pub struct AttackRaycast{
    base: Base<RayCast3D>,
    #[export] knockback_multiplier: f32,
    #[export] knockback_up_multiplier: f32,
    #[export] attack_damage: f64,
    #[export] display_mesh: Option<Gd<MeshInstance3D>>
}

#[godot_api]
impl IRayCast3D for AttackRaycast{
    fn physics_process(&mut self, _delta: f64){
        let collider = if let Some(c) = self.base().get_collider(){c}else{return;};
        if let Ok(mut other) = collider.try_cast::<CharacterBody3D>(){
            other.set_velocity((self.base().get_quaternion() * Vector3::LEFT) * self.knockback_multiplier + Vector3::UP * self.knockback_up_multiplier);
            self.base_mut().set_enabled(false);
            self.base_mut().set_physics_process(false);
            if let Some(mut health) = other.find_child(HEALTH_TRACKER_NAME).map(|v|v.cast::<TickingStatTracker>()){
                health.bind_mut().add_to_stat(-self.attack_damage);
            }
        }
    }
}

impl AttackRaycast{
    fn update_display_mesh_length(&mut self){
        
        self.display_mesh.as_mut().expect("no display mesh set")
            .set_scale(Vector3::new(1., 1., 1.));
    }
}