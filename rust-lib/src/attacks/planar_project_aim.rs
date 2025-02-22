use godot::builtin::Vector3;
use godot::classes::INode3D;
use godot::classes::Node3D;
use godot::classes::Viewport;
use godot::obj::Base; 
use godot::obj::Gd;
use godot::obj::WithBaseField;
use godot::prelude::godot_api;
use godot::prelude::GodotClass;

#[derive(GodotClass)]
#[class(base = Node3D, init)]
pub struct PlanarProjectAim{
    base: Base<Node3D>,
    #[export] plane_norm: Vector3,
    #[export] plane_point: Vector3
}

#[godot_api]
impl INode3D for PlanarProjectAim {
    fn process(&mut self, _delta: f64){
        let viewport = self.get_camera_viewport();
        let camera = viewport.get_camera_3d().unwrap();
        let screen_pos = viewport.get_mouse_position();

        // calculate ray direction using 2 points projected at different depths, this will account for how the camera projection is set up
        let ray_start = camera.project_position(screen_pos, 0.);
        let ray_dir = camera.project_position(screen_pos, 10.) - ray_start;
        
        let ray_length = self.plane_norm.dot(self.plane_point - ray_start) / self.plane_norm.dot(ray_dir);
        let point = ray_start + ray_dir * ray_length;
        self.base_mut().set_global_position(point);
    }
}

impl PlanarProjectAim{
    fn get_camera_viewport(&self) -> Gd<Viewport>{
        self.base()
            .get_viewport().expect("not in viewport")
    }
}