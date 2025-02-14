use godot::builtin::GString;
use godot::classes::InputEvent;
use godot::classes::Node;
use godot::global::godot_print;
use godot::obj::Base;
use godot::obj::Gd;
use godot::obj::WithBaseField;
use godot::prelude::godot_api;
use godot::prelude::GodotClass;
use godot::prelude::Vector3;

#[derive(GodotClass)]
#[class(base = Node, init)]
struct ClickToDelete{
    base: Base<Node>,
    #[export]
    action: GString
}

#[godot_api]
impl ClickToDelete{
    #[func]
    fn delete(&mut self){
        godot_print!("delete method called");
        self.base()
            .get_parent()
            .expect("can't run call to delete on object with no parent")
            .queue_free();
    }
    #[func]
    fn delete_on_click(&mut self, _camera: Gd<Node>, event: Gd<InputEvent>, _position: Vector3, _normal: Vector3, _shape_index: i32){
        if !event.is_action(self.action.arg()){
            return;
        }
        self.base()
            .get_parent()
            .expect("can't run call to delete on object with no parent")
            .queue_free();
    }
}