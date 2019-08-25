use specs::prelude::*;
use std::ops::Deref;
use crate::utilities::Matrix4f;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Transform(pub Matrix4f);

impl Deref for Transform {
    type Target = Matrix4f;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
