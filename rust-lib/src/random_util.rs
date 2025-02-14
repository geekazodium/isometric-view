use godot::builtin::Vector3;
use godot::classes::RandomNumberGenerator;
use godot::obj::Gd;

pub fn get_rand_pos(random: &mut Gd<RandomNumberGenerator>, p1: Vector3, p2: Vector3) -> Vector3{
    let x = random.randf_range(p1.x, p2.x);
    let y = random.randf_range(p1.y, p2.y);
    let z = random.randf_range(p1.z, p2.z);
    Vector3::new(x, y, z)
}