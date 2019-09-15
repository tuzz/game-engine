use specs::prelude::*;
use std::ops::Deref;
use crate::utilities::Vector3f;

#[derive(Component)]
#[storage(HashMapStorage)]
pub struct Specular(pub Vector3f);

impl Default for Specular {
    fn default() -> Self {
        Self(Vector3f::new(1.0, 1.0, 1.0))
    }
}

impl Deref for Specular {
    type Target = Vector3f;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
