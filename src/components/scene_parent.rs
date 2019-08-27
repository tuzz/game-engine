use specs::prelude::*;
use specs_hierarchy::Parent;
use std::ops::Deref;

pub struct SceneParent(pub Entity);

impl Component for SceneParent {
    type Storage = FlaggedStorage<Self, VecStorage<Self>>;
}

impl Parent for SceneParent {
    fn parent_entity(&self) -> Entity {
        self.0
    }
}

impl Deref for SceneParent {
    type Target = Entity;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
