use godot::builtin::Array;
use godot::builtin::Callable;
use godot::builtin::GString;
use godot::builtin::Quaternion;
use godot::builtin::Vector3;
use godot::classes::Area3D;
use godot::classes::CharacterBody3D;
use godot::classes::INode;
use godot::classes::Node;
use godot::classes::Node3D;
use godot::classes::PackedScene;
use godot::meta::FromGodot;
use godot::meta::ToGodot;
use godot::obj::Base;
use godot::obj::Gd;
use godot::obj::WithBaseField;
use godot::prelude::godot_api;
use godot::prelude::GodotClass;

use crate::behavior_state_machine::ai_behavior::AIBehaviorNode;

macro_rules! get_parent {
    ($($s: expr)?) => {
        $($s.base().get_parent().unwrap().cast::<AIBehaviorNode>())?
    }
}
macro_rules! get_enabled {
    ($($s: expr)?) => {
        $(get_parent!($s).bind_mut().is_selected($s.to_gd().clone().upcast()))?
    }
}
macro_rules! set_state {
    ($($s: expr, $b: expr)?) => {
        $(get_parent!($s).bind_mut().set_selected_state($b.clone()))?
    };
}
macro_rules! panic_message {
    ($($s: expr, $err: expr)?) => {
        $(format!("{} (from node: {})",$err,$s.base().get_path().to_string()).as_str())?
    };
}

#[derive(GodotClass)]
#[class(base = Node, init)]
pub struct EnemyIdleState{
    base: Base<Node>,
    #[export]
    detection_hitbox: Option<Gd<Area3D>>,
    #[export]
    detected_state: GString,
    #[export]
    target_tracker: Option<Gd<EnemyTargetTracker>>
}

#[godot_api]
impl INode for EnemyIdleState {
    fn ready(&mut self){
    }
    fn physics_process(&mut self, _delta: f64){
        if self.target_tracker.as_mut().map_or(false, |target_tracker|target_tracker.bind_mut().get_target_node().is_some()){
            set_state!(self,self.detected_state);
        }
    }
}

#[godot_api]
impl EnemyIdleState {
    #[func]
    fn on_body_entered_detection(&mut self, node: Gd<Node3D>){
        if !get_enabled!(self){return;}
        if let Some(target_tracker) = &mut self.target_tracker{
            target_tracker.bind_mut().set_target(node);
            set_state!(self,self.detected_state);
        }
    }
}

#[derive(GodotClass)]
#[class(base = Node, init)]
pub struct EnemyTargetingState{
    base: Base<Node>,
    #[export]
    distance_tolerance: f32,
    #[export]
    character_body: Option<Gd<CharacterBody3D>>,
    #[export]
    in_distance_state: GString,
    #[export]
    target_tracker: Option<Gd<EnemyTargetTracker>>,
    #[export]
    navigation_agent: Option<Gd<Node>>,
    #[export]
    follow_speed: f32
}

#[godot_api]
impl INode for EnemyTargetingState{
    fn ready(&mut self){
        
    }
    fn physics_process(&mut self, _delta: f64){
        let target_pos = self.get_target().get_global_position();
        self.set_nav_target_pos(target_pos);
        let mut character_body = self.get_character_body_expect();
        let dir = self.get_nav_agent_next_pos() - character_body.get_global_position();
        character_body.set_velocity(dir.normalized() * self.follow_speed);
        character_body.move_and_slide();
        if (target_pos - character_body.get_global_position()).length() < self.distance_tolerance{
            set_state!(self, self.in_distance_state);
        }
    }
}

impl EnemyTargetingState {
    fn get_character_body_expect(&self) -> Gd<CharacterBody3D>{
        self.get_character_body().expect(panic_message!(self,"character body not set!"))
    }
    fn get_target(&self) -> Gd<Node3D>{
        self.target_tracker.as_ref()
            .expect(panic_message!(self,"no target tracker")).bind()
            .get_target_node()
            .expect(panic_message!(self,"oh whops"))
    }
    fn set_nav_target_pos(&self, pos: Vector3){
        self.get_navigation_agent().expect("no nav agent set")
            .set("target_position", &pos.to_variant());
    }
    fn get_nav_agent_next_pos(&self) -> Vector3{
        Vector3::from_variant(&self.get_navigation_agent()
            .expect("no nav agent set")
            .callv("get_next_path_position", &Array::new()))
    }
}

const TREE_EXIT_SIGNAL: &str = "tree_exiting";
const INVALID_CALLABLE_ERR: &str = "callable may have been tampered with, something has gone seriously wrong";

#[derive(GodotClass)]
#[class(base = Node, init)]
struct EnemyAttackState{
    base: Base<Node>,
    attack_finished_callable: Option<Callable>,
    #[export]
    attack_finished_state: GString,
    #[export]
    target_tracker: Option<Gd<EnemyTargetTracker>>,
    #[export]
    attack_scene: Option<Gd<PackedScene>>,
    #[export]
    character_body: Option<Gd<CharacterBody3D>>,
    attack_used: bool
}

#[godot_api]
impl INode for EnemyAttackState {
    fn ready(&mut self){
        self.attack_finished_callable = Some(Callable::from_object_method(&self.to_gd(), "on_attack_finished"));
        self.attack_used = false;
    }
    fn physics_process(&mut self, _delta: f64){
        self.get_character_body().unwrap().move_and_slide();
        if self.attack_used { return; }
        if let Some(target) = self.get_target_tracker().map(|v| v.bind().get_target_node()).flatten(){
            let distance = self.get_character_body().unwrap().get_global_position() - target.get_position();
            let mut attack: Gd<Node3D> = self.get_attack_scene().unwrap().instantiate().expect("failed to instantiate").cast();
            self.base().get_parent().unwrap().add_child(&attack);
            attack.set_quaternion(Quaternion::from_rotation_arc(Vector3::new(1., 0., 0.), distance.normalized()));
            attack.connect(TREE_EXIT_SIGNAL, self.attack_finished_callable.as_ref().expect(INVALID_CALLABLE_ERR));
            self.attack_used = true;
        }else{
            set_state!(self, self.attack_finished_state);
        }
    }
}

#[godot_api]
impl EnemyAttackState{
    #[func]
    fn on_attack_finished(&mut self){
        self.attack_used = false;
        set_state!(self,self.attack_finished_state);
    }
}

#[derive(GodotClass)]
#[class(base = Node, init)]
struct EnemyTargetTracker{
    base: Base<Node>,
    #[export]
    target_node: Option<Gd<Node3D>>,
    target_invalid_method: Option<Callable>
}

#[godot_api]
impl INode for EnemyTargetTracker {
    fn ready(&mut self){
        self.target_invalid_method = Some(Callable::from_object_method(&self.to_gd(), "on_target_invalid"));
    }
}

#[godot_api]
impl EnemyTargetTracker{
    #[func]
    fn on_target_invalid(&mut self){
        self.target_node = None;
    }
}

impl EnemyTargetTracker{
    pub fn set_target(&mut self, mut target: Gd<Node3D>){
        if let Some(v) = self.target_node.as_mut(){
            v.disconnect(TREE_EXIT_SIGNAL, self.target_invalid_method.as_ref().expect(INVALID_CALLABLE_ERR));
        }
        target.connect(TREE_EXIT_SIGNAL, self.target_invalid_method.as_ref().expect(INVALID_CALLABLE_ERR));
        self.target_node = Some(target);
    }
}