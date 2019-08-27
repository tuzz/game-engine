use specs::prelude::*;
use specs_hierarchy::Hierarchy;
use crate::components::*;

pub struct SceneGraph;

impl<'a> System<'a> for SceneGraph {
    type SystemData = (
        ReadExpect<'a, Hierarchy<SceneParent>>,

        ReadStorage<'a, SceneParent>,
        ReadStorage<'a, LocalTransform>,

        WriteStorage<'a, WorldTransform>,
    );

    fn run(&mut self, (hierarchy, parents, local_transforms, mut world_transforms): Self::SystemData) {
        for entity in hierarchy.all() {
            let parent = parents.get(*entity).unwrap();

            let local = local_transforms.get(*entity).unwrap();
            let world = world_transforms.get(**parent).unwrap();

            let combined = WorldTransform(world.multiply(local));

            world_transforms.insert(*entity, combined).unwrap();
        }
    }
}
