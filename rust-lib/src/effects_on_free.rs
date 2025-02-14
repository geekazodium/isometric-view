use godot::classes::Node;
use godot::classes::Node3D;
use godot::classes::PackedScene;
use godot::global::godot_warn;
use godot::meta::ToGodot;
use godot::obj::Base;
use godot::obj::Gd;
use godot::obj::WithBaseField;
use godot::prelude::godot_api;
use godot::prelude::GodotClass;

#[derive(GodotClass)]
#[class(base = Node, init)]
struct EffectsOnDelete{
    base: Base<Node>,
    #[export]
    effects: Option<Gd<PackedScene>>
}

#[godot_api]
impl EffectsOnDelete{
    #[func]
    fn tree_exiting(&mut self){
        if let Some(effect_scene) = self.effects.as_ref(){
            let mut parent = self.base().get_parent().unwrap().get_parent().unwrap();
            let position = self.base().get_parent().unwrap().cast::<Node3D>().get_position();
            let mut instance: Gd<Node3D> = effect_scene.instantiate().expect("failed to instantiate effects").cast();
            parent.call_deferred("add_child",&[instance.to_variant()]);
            instance.set_position(position);
        }else{
            godot_warn!("no effects scene set!");
        }
    }
}