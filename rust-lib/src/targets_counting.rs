use godot::classes::CanvasLayer;
use godot::classes::INode3D;
use godot::classes::Node3D;
use godot::classes::Node;
use godot::global::godot_print;
use godot::obj::Base;
use godot::obj::Gd;
use godot::obj::WithBaseField;
use godot::prelude::godot_api;
use godot::prelude::GodotClass;

#[derive(GodotClass)]
#[class(base = Node3D, init)]
pub struct TargetsCounter{
    base: Base<Node3D>,
    sphere_count: i32,
    game_ended: bool
}

#[godot_api]
impl INode3D for TargetsCounter{
    fn ready(&mut self){
        self.sphere_count = self.base().get_child_count();
        self.game_ended = false;
    }
    fn process(&mut self, _delta: f64){
        self.sphere_count = self.base().get_child_count().max(self.sphere_count);
        //godot_print!("boop {}", self.base().get_child_count());
        if self.base().get_child_count() == 0{
            if self.game_ended{
                return;
            }
            self.game_ended = true;
            godot_print!("game ended");
            self.base_mut().emit_signal("on_completed", &[]);
        }
    }
}

#[godot_api]
impl TargetsCounter{
    #[signal]
    fn on_completed();
    #[func]
    pub fn get_sphere_count(&self)-> i32{
        self.sphere_count
    }
}

#[derive(GodotClass)]
#[class(base = Node, init)]
struct GameOverInit{
    base: Base<Node>,
    #[export]
    to_display: Option<Gd<CanvasLayer>>
}

#[godot_api]
impl GameOverInit{
    #[func]
    fn show_game_over_screen(&mut self){
        self.to_display.as_mut().expect("no scene to display").set_visible(true);
    }
}