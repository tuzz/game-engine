use specs::prelude::*;
use crate::utilities::Vector3f;

#[derive(Component)]
#[storage(HashMapStorage)]
pub struct DirectionalLight {
    pub direction_to_light: Vector3f,
}
