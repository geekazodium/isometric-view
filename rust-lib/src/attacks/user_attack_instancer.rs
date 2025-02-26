use godot::builtin::Callable;
use godot::builtin::GString;
use godot::builtin::Quaternion;
use godot::builtin::Vector3;
use godot::classes::INode3D;
use godot::classes::Input;
use godot::classes::Node3D;
use godot::classes::PackedScene;
use godot::meta::ToGodot;
use godot::obj::Base; 
use godot::obj::Gd;
use godot::obj::WithBaseField;
use godot::prelude::godot_api;
use godot::prelude::GodotClass;

#[derive(GodotClass)]
#[class(base = Node3D, init)]
pub struct UserAttackInstancer{
    base: Base<Node3D>,
    #[export] attack_action: GString,
    cooldown_timer: f64,
    #[export] cooldown: f64,
    attack_use_buffer_timer: f64,
    #[export] attack_use_buffer: f64,
    #[export] attack: Option<Gd<PackedScene>>,
    #[export] attack_rotate_from: Vector3,
    #[export] aim_pointer: Option<Gd<Node3D>>,
    attack_dealt_damage_callable: Option<Callable>,
    #[export] damage_signal: GString
}

#[godot_api]
impl INode3D for UserAttackInstancer{
    fn process(&mut self, _delta: f64){
        self.attack_dealt_damage_callable = Some(Callable::from_object_method(&self.to_gd(), "attack_damage_dealt"));
        if Input::singleton().is_action_pressed(self.attack_action.arg()){
            self.attack_use_buffer_timer = self.attack_use_buffer;
        }
    }
    fn physics_process(&mut self, delta: f64){
        if self.cooldown_timer > 0.{
            self.cooldown_timer -= delta;
        }else{
            if self.attack_use_buffer_timer > 0.{
                self.use_attack();
                self.attack_use_buffer_timer = 0.;
                self.cooldown_timer = self.cooldown;
            }
        }
        if self.attack_use_buffer_timer > 0.{
            self.attack_use_buffer_timer -= delta;
        }
    }
}

impl UserAttackInstancer{
    fn use_attack(&mut self){
        let mut attack: Gd<Node3D> = self.attack
            .as_ref().expect("no attack is set")
            .instantiate().expect("failed to instantiate scene")
            .try_cast().expect("scene is not instance of 3d scene");
        let aim_position = self.aim_pointer.as_ref().expect("no aim pointer set").get_global_position();
        let aim_dir = self.base().get_global_position() - aim_position;
        
        self.base_mut().add_child(&attack);
        attack.set_quaternion(Quaternion::from_rotation_arc(self.attack_rotate_from, if aim_dir.is_zero_approx(){
            Vector3::DOWN
        }else{
            aim_dir.normalized()
        }));
        attack.connect(&self.damage_signal.to_string(), self.attack_dealt_damage_callable.as_ref().unwrap());
    }
}

#[godot_api]
impl UserAttackInstancer {
    #[signal]
    fn instanced_attack_dealt_damage(amount: f64);
    #[func]
    fn attack_damage_dealt(&mut self, amount: f64){
        self.base_mut().emit_signal("instanced_attack_dealt_damage", &[amount.to_variant()]);
    }
}