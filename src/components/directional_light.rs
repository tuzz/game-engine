use specs::prelude::*;
use crate::utilities::Vector3f;

#[derive(Component)]
#[storage(HashMapStorage)]
pub struct DirectionalLight {
    pub direction_to_light: Vector3f,
}

impl DirectionalLight {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        let direction_to_light = Vector3f::new(x, y, z).normalize();

        Self { direction_to_light }
    }
}
