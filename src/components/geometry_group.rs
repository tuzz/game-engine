use specs::prelude::*;
use std::ops::Deref;

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct GeometryGroup(pub String);

impl Deref for GeometryGroup {
    type Target = String;

    fn deref(&self) -> &String {
        &self.0
    }
}

impl GeometryGroup {
    pub fn new(name: &str) -> Self {
        Self(name.to_string())
    }
}
