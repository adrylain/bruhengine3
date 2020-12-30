pub struct Vector3d{
	x: f32,
	y: f32,
	z: f32
}

impl Vector3d{
	pub fn new(x: f32, y: f32, z:f32) -> Vector3d{
		Vector3d{
			x:x,
			y:y,
			z:z,
		}
	}
}