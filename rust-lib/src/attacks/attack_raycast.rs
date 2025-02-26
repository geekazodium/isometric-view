use godot::classes::CharacterBody3D;
use godot::classes::IRayCast3D;
use godot::classes::MeshInstance3D;
use godot::classes::RayCast3D;
use godot::meta::ToGodot;
use godot::obj::Base;
use godot::obj::Gd;
use godot::obj::WithBaseField;
use godot::prelude::godot_api;
use godot::prelude::GodotClass;
use godot::prelude::Vector3;

use crate::game_meters::ticking_stat_tracker::TickingStatTracker;
use crate::game_meters::ticking_stat_tracker::HEALTH_TRACKER_NAME;

#[derive(GodotClass)]
#[class(base = RayCast3D, init)]
pub struct AttackRaycast{
    base: Base<RayCast3D>,
    #[export] knockback_multiplier: f32,
    #[export] knockback_up_multiplier: f32,
    #[export] attack_damage: f64,
    #[export] display_mesh: Option<Gd<MeshInstance3D>>,
    #[export] display_mesh_center: Vector3,
    hit: bool
}

#[godot_api]
impl IRayCast3D for AttackRaycast{
    fn physics_process(&mut self, _delta: f64){
        self.update_display_mesh_length();
        if self.hit {return;}
        let collider = if let Some(c) = self.base().get_collider(){
            c
        }else{
            return;
        };
        if let Ok(mut other) = collider.try_cast::<CharacterBody3D>(){
            other.set_velocity((self.base().get_quaternion() * Vector3::LEFT) * self.knockback_multiplier + Vector3::UP * self.knockback_up_multiplier);
            self.hit = true;
            if let Some(mut health) = other.find_child(HEALTH_TRACKER_NAME).map(|v|v.cast::<TickingStatTracker>()){
                health.bind_mut().add_to_stat(-self.attack_damage);
                let damage_variant = self.attack_damage.to_variant();
                self.base_mut().emit_signal("dealt_damage", &[damage_variant]);
            }
        }
    }
}

#[godot_api]
impl AttackRaycast{
    #[signal]
    fn dealt_damage(damage: f64);
}

impl AttackRaycast{
    fn update_display_mesh_length(&mut self){
        let distance_squared: f32 = if self.base().get_collider().is_some() {
            self.base().get_global_position().distance_squared_to(self.base().get_collision_point()) / self.base().get_position().distance_squared_to(self.base().get_target_position())
        }else{
            1.0
        };
        let distance = distance_squared.sqrt();
        let display_mesh = self.display_mesh.as_mut().expect("no display mesh set");
        display_mesh.set_scale(Vector3::new(distance, 1., 1.));
        display_mesh.set_position(self.display_mesh_center * distance);
        display_mesh.set_visible(true);
    }
}