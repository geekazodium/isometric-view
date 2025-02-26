use godot::obj::Base; 
use godot::obj::Gd;
use godot::prelude::godot_api;
use godot::prelude::GodotClass;
use godot::classes::Node;

use super::ticking_stat_tracker::TickingStatTracker;

pub const HEALTH_TRACKER_NAME: &str = "HealthTracker";

#[derive(GodotClass)]
#[class(base = Node, init)]
pub struct SanityHandler{
    base: Base<Node>,
    #[export] sanity: Option<Gd<TickingStatTracker>>,
    #[export] sanity_per_damage_taken: f64,
    #[export] sanity_per_damage_dealt: f64,
    #[export] sanity_per_kill: f64,
    #[export] damage_bonus_per_lost_sanity: f64
}

#[godot_api]
impl SanityHandler{
    #[func]
    fn on_health_modified(&mut self, amount: f64){
        if amount >= 0. {
            return;
        }
        self.add_to_sanity(amount.abs() * self.sanity_per_damage_taken);
    }
    #[func]
    fn on_deal_damage(&mut self, amount: f64){
        if amount <= 0.{
            return;
        }
        self.add_to_sanity(amount * self.sanity_per_damage_dealt);
    }
    #[func]
    fn on_kill(&mut self){
        self.add_to_sanity(self.sanity_per_kill);
    }
    #[func]
    pub fn get_bonus_damage(&self) -> f64{
        let sanity = self.sanity.as_ref()
            .expect("no sanity meter is set")
            .bind();
        (sanity.get_max_meter() - sanity.get_current_meter()) * self.damage_bonus_per_lost_sanity
    }
}

impl SanityHandler{
    fn add_to_sanity(&mut self, amount: f64){
        self.sanity.as_mut()
            .expect("no sanity meter is set")
            .bind_mut().add_to_stat(amount);
    }
}