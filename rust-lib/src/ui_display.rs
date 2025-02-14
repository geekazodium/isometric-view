use godot::builtin::GString;
use godot::classes::RichTextLabel;
use godot::obj::Base;
use godot::obj::Gd;
use godot::obj::WithBaseField;
use godot::prelude::godot_api;
use godot::prelude::GodotClass;

use crate::targets_counting::TargetsCounter;

#[derive(GodotClass)]
#[class(base = RichTextLabel, init)]
struct BallsCountDisplay{
    base: Base<RichTextLabel>,
    #[export]
    target_counter: Option<Gd<TargetsCounter>>,
    #[export]
    base_text: GString
}

#[godot_api]
impl BallsCountDisplay {
    #[func]
    fn update(&mut self){
        let text = format!("{} {}",self.base_text, self.target_counter.as_ref().unwrap().bind().get_sphere_count());
        self.base_mut().set_text(&text);
    }
}