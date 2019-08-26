mod convert;
mod add;
mod subtract;
mod scale;
mod cross;

use convert::Tuple;

#[derive(Default)]
pub struct Vector3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3f {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}
