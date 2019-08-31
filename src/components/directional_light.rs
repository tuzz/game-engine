use specs::prelude::*;
use crate::utilities::Vector3f;

#[derive(Component)]
#[storage(HashMapStorage)]
pub struct DirectionalLight {
    pub reverse_light_direction: Vector3f,
}
