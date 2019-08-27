use specs::prelude::*;

#[derive(Component, Default)]
#[storage(HashMapStorage)]
pub struct ClearColor(pub f32, pub f32, pub f32, pub f32);
