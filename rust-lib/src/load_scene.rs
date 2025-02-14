use godot::builtin::GString;
use godot::classes::Node3D;
use godot::obj::Base;
use godot::obj::WithBaseField;
use godot::prelude::godot_api;
use godot::prelude::GodotClass;

#[derive(GodotClass)]
#[class(base = Node3D, init)]
struct LoadScene{
    base: Base<Node3D>,
    #[export]
    next: GString
}

#[godot_api]
impl LoadScene{
    #[func]
    fn load_next_scene(&mut self){
        let str = self.next.to_string();
        self.base().get_tree().unwrap().change_scene_to_file(&str);
    }
}