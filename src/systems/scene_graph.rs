use specs::prelude::*;
use specs_hierarchy::{Hierarchy, HierarchyEvent};
use crate::components::*;

#[derive(Default)]
pub struct SceneGraph {
    dirty: BitSet,

    local_transform_reader: Option<ReaderId<ComponentEvent>>,
    scene_parent_reader: Option<ReaderId<HierarchyEvent>>,
}

type Hier = Hierarchy<SceneParent>;

#[derive(SystemData)]
pub struct SysData<'a> {
    entities: Entities<'a>,
    parents: WriteStorage<'a, SceneParent>,
    locals: ReadStorage<'a, LocalTransform>,
    worlds: WriteStorage<'a, WorldTransform>,
}

impl<'a> System<'a> for SceneGraph {
    type SystemData = (ReadExpect<'a, Hier>, SysData<'a>);

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        self.local_transform_reader = Some(
            WriteStorage::<LocalTransform>::fetch(world).register_reader()
        );

        self.scene_parent_reader = Some(
            world.get_mut::<Hierarchy<SceneParent>>().unwrap().track()
        );
    }

    fn run(&mut self, (hierarchy, mut s): Self::SystemData) {
        self.mark_entities_that_have_moved_as_dirty(&mut s);
        self.mark_entities_that_have_changed_parent_as_dirty(&hierarchy);

        self.remove_world_transforms_from_entities_without_locals(&mut s);
        self.ensure_root_entities_have_up_to_date_world_transforms(&mut s);

        for child in hierarchy.all() {
            self.update_world_transform_if_stale(child, &mut s);
        }
    }
}

impl<'a> SceneGraph {
    fn mark_entities_that_have_moved_as_dirty(&mut self, s: &mut SysData) {
        self.dirty.clear();

        let reader_id = self.local_transform_reader.as_mut().unwrap();
        for event in s.locals.channel().read(reader_id) {
            match event {
                ComponentEvent::Modified(id) => { self.dirty.add(*id); },
                ComponentEvent::Inserted(id) => { self.dirty.add(*id); },
                ComponentEvent::Removed(id)  => { self.dirty.add(*id); },
            };
        }
    }

    fn mark_entities_that_have_changed_parent_as_dirty(&mut self, hierarchy: &Hier) {
        let reader_id = self.scene_parent_reader.as_mut().unwrap();
        for event in hierarchy.changed().read(reader_id) {
            match event {
                HierarchyEvent::Modified(entity) => { self.dirty.add(entity.id()); },
                HierarchyEvent::Removed(entity)  => { self.dirty.add(entity.id()); },
            }
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
    use crate::systems::Hierarchy;
    use crate::utilities::Matrix4f;

    fn setup() -> (World, Hierarchy, SceneGraph) {
        let mut world = World::new();

        let hierarchy = Hierarchy::new(&mut world);
        let mut scene_graph = SceneGraph::default();

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

        drop(read);

        world.entities_mut().delete(grandparent).unwrap();
        world.maintain();

        hierarchy.run_now(&mut world);
        scene_graph.run_now(&mut world);

        let read = world.read_storage::<WorldTransform>();

        assert_eq!(read.get(grandparent), None);
        assert_eq!(*read.get(parent).unwrap(), WorldTransform(Matrix4f::translation(4., 5., 6.)));
        assert_eq!(read.get(child), None);
    }
}
