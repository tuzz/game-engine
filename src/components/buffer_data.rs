use specs::prelude::*;
use std::ops::Deref;

#[derive(Component)]
#[storage(HashMapStorage)]
pub struct BufferData(Vec<f32>);

impl Deref for BufferData {
    type Target = Vec<f32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
