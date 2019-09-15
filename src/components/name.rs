use specs::prelude::*;
use std::ops::Deref;

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Name(pub String);

impl Deref for Name {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
