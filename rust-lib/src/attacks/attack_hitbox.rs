use godot::classes::Area3D;
use godot::classes::CharacterBody3D;
use godot::classes::IArea3D;
use godot::obj::Base;
use godot::obj::WithBaseField;
use godot::prelude::godot_api;
use godot::prelude::GodotClass;
use godot::prelude::Vector3;

use crate::game_meters::ticking_stat_tracker::TickingStatTracker;
use crate::game_meters::ticking_stat_tracker::HEALTH_TRACKER_NAME;

#[derive(GodotClass)]
#[class(base = Area3D, init)]
struct AttackHitbox{
    base: Base<Area3D>,
    #[export] knockback_up_multiplier: f32,
    #[export] attack_damage: f64,
    #[export] knockback_direction: Vector3,
    hit: bool,
    #[export] persistent: bool
}

#[godot_api]
impl IArea3D for AttackHitbox {
    fn physics_process(&mut self, _delta: f64){
        if !self.persistent && self.hit{
            return;
        }
        for collider in self.base().get_overlapping_bodies().iter_shared(){
            if let Ok(mut other) = collider.try_cast::<CharacterBody3D>(){
                self.hit = true;
                other.set_velocity((self.base().get_quaternion() * self.knockback_direction) + Vector3::UP * self.knockback_up_multiplier);
                if let Some(mut health) = other.find_child(HEALTH_TRACKER_NAME).map(|v|v.cast::<TickingStatTracker>()){
                    health.bind_mut().add_to_stat(-self.attack_damage);
                }
            }
        }
    }
}