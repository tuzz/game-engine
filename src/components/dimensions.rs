use specs::prelude::*;
use std::ops::Deref;

#[derive(Component, Clone)]
#[storage(VecStorage)]
pub struct Dimensions(pub u32);

impl Deref for Dimensions {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
