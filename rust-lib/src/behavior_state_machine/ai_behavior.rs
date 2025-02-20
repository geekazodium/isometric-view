use godot::builtin::Dictionary;
use godot::builtin::GString;
use godot::classes::INode3D;
use godot::classes::Node;
use godot::classes::Node3D;
use godot::obj::Base;
use godot::obj::Gd;
use godot::obj::WithBaseField;
use godot::prelude::godot_api;
use godot::prelude::GodotClass;

#[derive(GodotClass)]
#[class(base = Node3D, init)]
pub struct AIBehaviorNode{
    base: Base<Node3D>,
    #[export]
    states_dict: Dictionary,
    #[export]
    current_state: GString
}

#[godot_api]
impl INode3D for AIBehaviorNode{
    fn ready(&mut self){
        self.init_state_machine();
        self.notify_processing(true);
    }
}

#[godot_api]
impl AIBehaviorNode {
    #[func]
    pub fn set_selected_state(&mut self, game_state: GString){
        self.notify_processing(false);
        self.current_state = game_state;
        self.notify_processing(true);
    }
    #[func]
    pub fn is_selected(&self, node: Gd<Node>) -> bool{
        self.get_selected_state().eq(&node)
    }
}

impl AIBehaviorNode{
    fn init_state_machine(&mut self){
        self.base().get_children()
            .iter_shared()
            .for_each(|mut c|{
                c.set_process(false);
                c.set_physics_process(false);
            });
    }
    fn get_selected_state(&self) -> Gd<Node>{
        let c = self.states_dict.get(self.current_state.to_string())
            .expect("state not set!");
        self.base().find_child(&c.to_string()).expect("hacky solution did not work!")
    }
    fn notify_processing(&mut self, processing: bool){
        //godot_print!("notified state change for {} to state: {}",self.get_selected_state().to_string(), if processing {"enabled"} else {"disabled"});
        let mut selected_state = self.get_selected_state();
        selected_state.set_process(processing);
        selected_state.set_physics_process(processing);
    }
}