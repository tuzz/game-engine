use specs::prelude::*;
use specs_hierarchy::Parent;

pub struct SceneParent(pub Entity);

impl Component for SceneParent {
    type Storage = FlaggedStorage<Self, VecStorage<Self>>;
}

impl Parent for SceneParent {
    fn parent_entity(&self) -> Entity {
        self.0
    }
}
