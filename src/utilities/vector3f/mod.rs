mod convert;
mod cross;

pub struct Vector3f(pub [f32; 3]);

type Tuple = (f32, f32, f32);

impl Vector3f {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self([x, y, z])
    }
}
