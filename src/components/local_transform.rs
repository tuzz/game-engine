use specs::prelude::*;
use std::ops::{Deref, DerefMut};
use crate::utilities::Matrix4f;

pub struct LocalTransform(pub Matrix4f);

impl<'a> Component for LocalTransform {
    type Storage = FlaggedStorage<Self, VecStorage<Self>>;
}

impl Deref for LocalTransform {
    type Target = Matrix4f;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LocalTransform {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
