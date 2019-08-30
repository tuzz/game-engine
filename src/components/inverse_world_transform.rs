use specs::prelude::*;
use std::ops::{Deref, DerefMut};
use crate::utilities::Matrix4f;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct InverseWorldTransform(pub Matrix4f);

impl Deref for InverseWorldTransform {
    type Target = Matrix4f;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for InverseWorldTransform {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
