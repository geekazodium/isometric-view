use godot::builtin::Callable;
use godot::classes::Control;
use godot::classes::INode;
use godot::classes::Node;
use godot::obj::Base;
use godot::obj::Gd;
use godot::obj::WithBaseField;
use godot::prelude::godot_api;
use godot::prelude::GodotClass;

use crate::event_bus::EventBus;

#[derive(GodotClass)]
#[class(base = Node, init)]
struct PlayerLoseHandler{
    base: Base<Node>,
    #[export] game_over_scene: Option<Gd<Control>>
}

#[godot_api]
impl INode for PlayerLoseHandler {
    fn ready(&mut self){
        EventBus::singleton().connect("player_health_changed", &Callable::from_object_method(&self.to_gd(), "on_player_health_change"));
        EventBus::singleton().connect("player_sanity_changed", &Callable::from_object_method(&self.to_gd(), "on_player_health_change"));
    }
}

#[godot_api]
impl PlayerLoseHandler{
    #[func]
    fn on_player_health_change(&mut self, new_health: f64){
        if new_health > 0.{
            return;
        }
        self.game_over_scene
            .as_mut().expect("no game over scene is set")
            .set_visible(true);
    }
}