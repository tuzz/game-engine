use specs::prelude::*;
use std::ops::Deref;

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct ModelGroup(pub String);

impl Deref for ModelGroup {
    type Target = String;

    fn deref(&self) -> &String {
        &self.0
    }
}

impl ModelGroup {
    pub fn new(name: &str) -> Self {
        Self(name.to_string())
    }
}
