use specs::prelude::*;
use specs_hierarchy::Hierarchy;
use crate::components::*;

#[derive(Default)]
pub struct SceneGraph {
    dirty: BitSet,
    reader_id: Option<ReaderId<ComponentEvent>>,
}

#[derive(SystemData)]
pub struct SysData<'a> {
    entities: Entities<'a>,
    parents: ReadStorage<'a, SceneParent>,
    locals: ReadStorage<'a, LocalTransform>,
    worlds: WriteStorage<'a, WorldTransform>,
}

impl<'a> System<'a> for SceneGraph {
    type SystemData = (
        ReadExpect<'a, Hierarchy<SceneParent>>,
        SysData<'a>,
    );

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        self.reader_id = Some(
            WriteStorage::<LocalTransform>::fetch(world).register_reader()
        );
    }

    fn run(&mut self, (hierarchy, mut s): Self::SystemData) {
        self.mark_entities_that_have_moved_as_dirty(&s);

        self.remove_world_transforms_from_entities_without_locals(&mut s);
        self.ensure_root_entities_have_up_to_date_world_transforms(&mut s);

        for child in hierarchy.all() {
            self.update_world_transform_if_stale(child, &mut s);
        }
    }
}

// TODO: what if scene parent is removed?

impl<'a> SceneGraph {
    fn mark_entities_that_have_moved_as_dirty(&mut self, s: &SysData) {
        self.dirty.clear();

        for event in s.locals.channel().read(self.reader_id.as_mut().unwrap()) {
            match event {
                ComponentEvent::Modified(id) => { self.dirty.add(*id); },
                ComponentEvent::Inserted(id) => { self.dirty.add(*id); },
                ComponentEvent::Removed(id)  => { self.dirty.add(*id); },
            };
        }
    }

    fn remove_world_transforms_from_entities_without_locals(&mut self, s: &mut SysData) {
        (!&s.locals, &s.worlds, &s.entities).join().map(|(_, _, e)| e)
            .collect::<Vec<_>>().iter().for_each(|&e| { s.worlds.remove(e); });
    }

    fn ensure_root_entities_have_up_to_date_world_transforms(&mut self, s: &mut SysData) {
        for (_, _, local, root) in (&self.dirty, !&s.parents, &s.locals, &s.entities).join() {
            let world = WorldTransform(local.0.clone());

            s.worlds.insert(root, world).unwrap();
        }
    }

    fn update_world_transform_if_stale(&mut self, child: &Entity, s: &mut SysData) -> Option<()> {
        let parent = s.parents.get(*child)?;

        match self.dirty.contains(parent.id()) {
            true => self.dirty.add(child.id()),
            false => self.dirty.contains(child.id()) || return None,
        };

        let local = s.locals.get(*child)?;

        // Remove the world transform from the child if the parent doesn't have one.
        let world = match s.worlds.get(**parent) {
            Some(w) => w,
            None => {
                s.worlds.remove(*child).unwrap();
                return None;
            }
        };

        let combined = WorldTransform(world.multiply(local));
        s.worlds.insert(*child, combined).unwrap();

        Some(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use specs_hierarchy::HierarchySystem;
    use crate::utilities::Matrix4f;

    fn setup() -> (World, HierarchySystem<SceneParent>, SceneGraph) {
        let mut world = World::new();

        let mut hierarchy = HierarchySystem::<SceneParent>::new(&mut world);
        let mut scene_graph = SceneGraph::default();

        System::setup(&mut hierarchy, &mut world);
        System::setup(&mut scene_graph, &mut world);

        (world, hierarchy, scene_graph)
    }

    #[test]
    fn it_adds_a_world_transform_component_to_nodes_with_local_transforms() {
        let (mut world, mut hierarchy, mut scene_graph) = setup();

        let entity = world.create_entity()
            .with(LocalTransform(Matrix4f::identity()))
            .build();

        hierarchy.run_now(&mut world);
        scene_graph.run_now(&mut world);

        let storage = world.read_storage::<WorldTransform>();
        let transform = storage.get(entity).unwrap();

        assert_eq!(*transform, WorldTransform(Matrix4f::identity()));
    }
}
