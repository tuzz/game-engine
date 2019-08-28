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
                s.worlds.remove(*child);
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
    fn it_sets_the_world_transforms_by_recursing_down_the_hierarchy() {
        let (mut world, mut hierarchy, mut scene_graph) = setup();

        let grandparent = world.create_entity()
            .with(LocalTransform(Matrix4f::translation(1., 2., 3.)))
            .build();

        let parent = world.create_entity()
            .with(LocalTransform(Matrix4f::translation(4., 5., 6.)))
            .with(SceneParent(grandparent))
            .build();

        let child = world.create_entity()
            .with(LocalTransform(Matrix4f::translation(7., 8., 9.)))
            .with(SceneParent(parent))
            .build();

        hierarchy.run_now(&mut world);
        scene_graph.run_now(&mut world);

        let read = world.read_storage::<WorldTransform>();

        assert_eq!(*read.get(grandparent).unwrap(), WorldTransform(Matrix4f::translation(1., 2., 3.)));
        assert_eq!(*read.get(parent).unwrap(), WorldTransform(Matrix4f::translation(5., 7., 9.)));
        assert_eq!(*read.get(child).unwrap(), WorldTransform(Matrix4f::translation(12., 15., 18.)));
    }

    #[test]
    fn it_updates_the_world_transforms_when_local_transforms_change() {
        let (mut world, mut hierarchy, mut scene_graph) = setup();

        let grandparent = world.create_entity()
            .with(LocalTransform(Matrix4f::translation(1., 2., 3.)))
            .build();

        let parent = world.create_entity()
            .with(LocalTransform(Matrix4f::translation(4., 5., 6.)))
            .with(SceneParent(grandparent))
            .build();

        let child = world.create_entity()
            .with(LocalTransform(Matrix4f::translation(7., 8., 9.)))
            .with(SceneParent(parent))
            .build();

        hierarchy.run_now(&mut world);
        scene_graph.run_now(&mut world);

        let mut write = world.write_storage::<LocalTransform>();
        *write.get_mut(grandparent).unwrap() = LocalTransform(Matrix4f::identity());

        drop(write);

        hierarchy.run_now(&mut world);
        scene_graph.run_now(&mut world);

        let read = world.read_storage::<WorldTransform>();

        assert_eq!(*read.get(grandparent).unwrap(), WorldTransform(Matrix4f::identity()));
        assert_eq!(*read.get(parent).unwrap(), WorldTransform(Matrix4f::translation(4., 5., 6.)));
        assert_eq!(*read.get(child).unwrap(), WorldTransform(Matrix4f::translation(11., 13., 15.)));

        let mut write = world.write_storage::<LocalTransform>();
        *write.get_mut(child).unwrap() = LocalTransform(Matrix4f::identity());

        drop(read); drop(write);

        hierarchy.run_now(&mut world);
        scene_graph.run_now(&mut world);

        let read = world.read_storage::<WorldTransform>();

        assert_eq!(*read.get(grandparent).unwrap(), WorldTransform(Matrix4f::identity()));
        assert_eq!(*read.get(parent).unwrap(), WorldTransform(Matrix4f::translation(4., 5., 6.)));
        assert_eq!(*read.get(child).unwrap(), WorldTransform(Matrix4f::translation(4., 5., 6.)));

        let mut write = world.write_storage::<LocalTransform>();
        *write.get_mut(parent).unwrap() = LocalTransform(Matrix4f::identity());

        drop(read); drop(write);

        hierarchy.run_now(&mut world);
        scene_graph.run_now(&mut world);

        let read = world.read_storage::<WorldTransform>();

        assert_eq!(*read.get(grandparent).unwrap(), WorldTransform(Matrix4f::identity()));
        assert_eq!(*read.get(parent).unwrap(), WorldTransform(Matrix4f::identity()));
        assert_eq!(*read.get(child).unwrap(), WorldTransform(Matrix4f::identity()));
    }

    #[test]
    fn it_removes_world_transforms_when_local_transforms_are_removed() {
        let (mut world, mut hierarchy, mut scene_graph) = setup();

        let grandparent = world.create_entity()
            .with(LocalTransform(Matrix4f::translation(1., 2., 3.)))
            .build();

        let parent = world.create_entity()
            .with(LocalTransform(Matrix4f::translation(4., 5., 6.)))
            .with(SceneParent(grandparent))
            .build();

        let child = world.create_entity()
            .with(LocalTransform(Matrix4f::translation(7., 8., 9.)))
            .with(SceneParent(parent))
            .build();

        hierarchy.run_now(&mut world);
        scene_graph.run_now(&mut world);

        let mut write = world.write_storage::<LocalTransform>();
        write.remove(grandparent).unwrap();

        drop(write);

        hierarchy.run_now(&mut world);
        scene_graph.run_now(&mut world);

        let read = world.read_storage::<WorldTransform>();

        assert_eq!(read.get(grandparent), None);
        assert_eq!(read.get(parent), None);
        assert_eq!(read.get(child), None);
    }

    #[test]
    fn it_does_not_add_world_transforms_if_the_parent_doesnt_have_one() {
        let (mut world, mut hierarchy, mut scene_graph) = setup();

        let parent = world.create_entity()
            .build();

        let child = world.create_entity()
            .with(LocalTransform(Matrix4f::translation(4., 5., 6.)))
            .with(SceneParent(parent))
            .build();

        hierarchy.run_now(&mut world);
        scene_graph.run_now(&mut world);

        let mut write = world.write_storage::<LocalTransform>();
        *write.get_mut(child).unwrap() = LocalTransform(Matrix4f::identity());

        drop(write);

        hierarchy.run_now(&mut world);
        scene_graph.run_now(&mut world);

        let read = world.read_storage::<WorldTransform>();

        assert_eq!(read.get(parent), None);
        assert_eq!(read.get(child), None);
    }

    #[test]
    fn it_updates_world_transforms_when_scene_parents_change() {
        // TODO: make this test pass
        let (mut world, mut hierarchy, mut scene_graph) = setup();

        let grandparent = world.create_entity()
            .with(LocalTransform(Matrix4f::translation(1., 2., 3.)))
            .build();

        let parent = world.create_entity()
            .with(LocalTransform(Matrix4f::translation(4., 5., 6.)))
            .with(SceneParent(grandparent))
            .build();

        let child = world.create_entity()
            .with(LocalTransform(Matrix4f::translation(7., 8., 9.)))
            .with(SceneParent(parent))
            .build();

        hierarchy.run_now(&mut world);
        scene_graph.run_now(&mut world);

        let mut write = world.write_storage::<SceneParent>();
        write.remove(parent).unwrap();

        drop(write);

        hierarchy.run_now(&mut world);
        scene_graph.run_now(&mut world);

        let read = world.read_storage::<WorldTransform>();

        assert_eq!(*read.get(grandparent).unwrap(), WorldTransform(Matrix4f::translation(1., 2., 3.)));
        assert_eq!(*read.get(parent).unwrap(), WorldTransform(Matrix4f::translation(4., 5., 6.)));
        assert_eq!(*read.get(child).unwrap(), WorldTransform(Matrix4f::translation(11., 13., 15.)));

        let mut write = world.write_storage::<SceneParent>();
        *write.get_mut(child).unwrap() = SceneParent(grandparent);

        drop(read); drop(write);

        hierarchy.run_now(&mut world);
        scene_graph.run_now(&mut world);

        let read = world.read_storage::<WorldTransform>();

        assert_eq!(*read.get(grandparent).unwrap(), WorldTransform(Matrix4f::translation(1., 2., 3.)));
        assert_eq!(*read.get(parent).unwrap(), WorldTransform(Matrix4f::translation(4., 5., 6.)));
        assert_eq!(*read.get(child).unwrap(), WorldTransform(Matrix4f::translation(8., 10., 12.)));
    }

    // TODO: remove the parent entity
}
