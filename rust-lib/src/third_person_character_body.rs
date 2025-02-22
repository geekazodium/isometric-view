use core::f32;

use godot::builtin::GString;
use godot::builtin::Vector2;
use godot::builtin::Vector3;
use godot::classes::CharacterBody3D;
use godot::classes::ICharacterBody3D;
use godot::classes::INode;
use godot::classes::Input;
use godot::classes::Node;
use godot::meta::ToGodot;
use godot::obj::Base;
use godot::obj::Gd;
use godot::obj::WithBaseField;
use godot::prelude::godot_api;
use godot::prelude::GodotClass;

#[derive(GodotClass)]
#[class[base = CharacterBody3D, init]]
struct ThirdPersonCharacterBody{
    base: Base<CharacterBody3D>,
    #[export] acceleration: f32,
    
    #[export] forward: Vector3,
    #[export] left: Vector3,

    #[export] forward_back_opposing_actions: Option<Gd<OpposingKeyboardActions>>,
    #[export] left_right_opposing_actions: Option<Gd<OpposingKeyboardActions>>,
    #[export] grounded_movement: Option<Gd<GroundedMovement>>
}


#[godot_api]
impl ICharacterBody3D for ThirdPersonCharacterBody {
    fn physics_process(&mut self, delta: f64){
        let forward = self.get_forward();

        let left = self.get_left();

        let move_dir = self.get_direction_vec();
        let direction = forward * move_dir.y + left * move_dir.x;

        self.get_grounded_movement().unwrap().bind().move_grounded(self.base().to_godot(), direction * self.acceleration, delta as f32);
    }
}

impl ThirdPersonCharacterBody{
    fn get_direction_vec(&self) -> Vector2{
        Vector2::new(
            self.left_right_opposing_actions.as_ref().expect("left right actions not set").bind().get_direction(), 
            self.forward_back_opposing_actions.as_ref().expect("forward back actions not set").bind().get_direction()
        )
        .normalized_or_zero()
    }
}

#[derive(GodotClass)]
#[class(base = Node,init)]
struct OpposingKeyboardActions{
    base: Base<Node>,
    #[export] positive_action: GString,
    #[export] negative_action: GString,

    direction: i8
}

#[godot_api]
impl INode for OpposingKeyboardActions{
    fn process(&mut self, _delta: f64){
        if Input::singleton().is_action_just_pressed(self.positive_action.arg()){
            self.direction = 1;
        }
        if Input::singleton().is_action_just_pressed(self.negative_action.arg()){
            self.direction = -1;
        }
        if Input::singleton().is_action_just_released(self.positive_action.arg()){
            self.direction = if Input::singleton().is_action_pressed(self.negative_action.arg()) { -1 } else { 0 };
        }
        if Input::singleton().is_action_just_released(self.negative_action.arg()){
            self.direction = if Input::singleton().is_action_pressed(self.positive_action.arg()) { 1 } else { 0 };
        }
    }
}

impl OpposingKeyboardActions {
    pub fn get_direction(&self) -> f32{
        self.direction as f32
    }
}

#[derive(GodotClass)]
#[class(base = Node, init)]
pub struct GroundedMovement{
    base: Base<Node>,
    #[export] gravity: f32,
    #[export] ground_friction: f32
}

impl GroundedMovement{
    pub fn move_grounded(&self, mut character_body: Gd<CharacterBody3D>, move_dir: Vector3, delta: f32){
        let velocity = character_body.get_velocity();
        let mut delta_v = Vector3::ZERO;
        delta_v += character_body.get_up_direction() * self.gravity;
        if character_body.is_on_floor(){
            delta_v += -velocity * self.ground_friction;
            delta_v += move_dir;
        }
        character_body.set_velocity(velocity + delta_v * delta);
        character_body.move_and_slide();
    }
}