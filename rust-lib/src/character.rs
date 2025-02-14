use godot::builtin::GString;
use godot::builtin::Vector2;
use godot::classes::CharacterBody2D;
use godot::classes::ICharacterBody2D;
use godot::classes::Input;
use godot::obj::Base;
use godot::obj::WithBaseField;
use godot::prelude::godot_api;
use godot::prelude::GodotClass;

#[derive(GodotClass)]
#[class(base = CharacterBody2D, init)]
struct PlayerCharacterBody{
    base: Base<CharacterBody2D>,
    #[export]
    horizontal_accel_speed: f32,
    #[export]
    gravity: f32,
    #[export]
    max_walk_speed: f32,
    #[export]
    jump_y_vel: f32,
    #[export]
    reverse_boost: f32,
    #[export]
    air_friction: Vector2,
    #[export]
    leading_jump_buffer: f64,
    jump_timer: f64,
    #[export]
    trailing_jump_buffer: f64,
    ground_timer: f64,
    #[export]
    move_left_action: GString,
    #[export]
    move_right_action: GString,
    #[export]
    jump_input: GString
}

#[godot_api]
impl ICharacterBody2D for PlayerCharacterBody {
    fn process(&mut self, _delta: f64){
        if Input::singleton().is_action_just_pressed(self.jump_input.arg()){
            self.jump_timer = self.leading_jump_buffer;
        }
    }
    fn physics_process(&mut self, delta: f64){
        if self.jump_timer > 0.{
            self.jump_timer -= delta;
        }
        if self.ground_timer > 0.{
            self.ground_timer -= delta;
        }
        if self.base().is_on_floor(){
            self.ground_timer = self.trailing_jump_buffer;
        }
        let up = self.base().get_up_direction();
        let left = Vector2::new(up.y, -up.x);
        let delta_time_f32 = delta as f32;
        let velocity = self.base().get_velocity();
        
        let mut delta_v = Vector2::new(0., 0.);
        if Input::singleton().is_action_pressed(self.move_left_action.arg()){
            let distance_from_max_v = self.max_walk_speed - left.dot(velocity);
            if distance_from_max_v > 0.{
                if distance_from_max_v < self.horizontal_accel_speed * delta_time_f32 {
                    delta_v += left * (distance_from_max_v / delta_time_f32);
                }else{
                    delta_v += self.horizontal_accel_speed * left;
                }
            }
        }else if Input::singleton().is_action_pressed(self.move_right_action.arg()){
            let distance_from_max_v = self.max_walk_speed + left.dot(velocity);
            if distance_from_max_v > 0.{
                if distance_from_max_v < self.horizontal_accel_speed * delta_time_f32 {
                    delta_v += -left * (distance_from_max_v / delta_time_f32);
                }else{
                    delta_v += -self.horizontal_accel_speed * left;
                }
            }
        }else{
            delta_v += -velocity.project(left) * self.reverse_boost;
        }
        delta_v.y += self.get_gravity();
        delta_v += -velocity * self.get_air_friction();

        let mut delta_v_instant = Vector2::new(0., 0.);

        if self.jump_timer > 0. && self.ground_timer > 0.{
            self.jump_timer = 0.;
            self.ground_timer = 0.;
            delta_v_instant += up * self.jump_y_vel;
            delta_v_instant += -velocity.project(up);
        }

        //apply delta velocity
        self.base_mut().set_velocity(velocity + delta_v * delta_time_f32 + delta_v_instant);
        self.base_mut().move_and_slide();
    }
}
