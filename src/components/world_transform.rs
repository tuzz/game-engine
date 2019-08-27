use specs::prelude::*;
use std::ops::{Deref, DerefMut};
use crate::utilities::Matrix4f;

#[derive(Component)]
#[storage(VecStorage)]
pub struct WorldTransform(pub Matrix4f);

impl Deref for WorldTransform {
    type Target = Matrix4f;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for WorldTransform {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
