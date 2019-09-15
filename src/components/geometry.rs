use specs::prelude::*;
use crate::resources::NameIndex;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Geometry {
    pub model: Entity,
}

impl Geometry {
    pub fn find(index: &NameIndex, name: &str) -> Self {
        let key = format!("geometry_{}", name);
        let model = *index.get(&key).unwrap();

        Self { model }
    }
}
