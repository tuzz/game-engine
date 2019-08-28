use specs::prelude::*;
use specs_hierarchy::{Hierarchy as Inner, HierarchyEvent, HierarchySystem};
use crate::components::*;

// From the specs-hierarchy docs:
//
// > When an Entity that is a parent gets removed from the hierarchy, the
// > full tree of children below it will also be removed from the hierarchy.
//
// This doesn't seem right, so re-add these entities to the hierarchy by
// adding and removing the SceneParent component.
//
// Related: https://github.com/amethyst/amethyst/issues/1549

pub struct Hierarchy {
    hierarchy_system: HierarchySystem::<SceneParent>,
    reader_id: ReaderId<HierarchyEvent>,
}

type SystemData<'a> = (
    Entities<'a>,
    ReadExpect<'a, Inner<SceneParent>>,
    WriteStorage<'a, SceneParent>,
);

impl Hierarchy {
    pub fn new(world: &mut World) -> Self {
        let mut hierarchy_system = HierarchySystem::<SceneParent>::new(world);

        System::setup(&mut hierarchy_system, world);

        let reader_id = world.get_mut::<Inner<SceneParent>>().unwrap().track();

        Self { hierarchy_system, reader_id }
    }

    pub fn run_now(&mut self, world: &mut World) {
        self.hierarchy_system.run_now(world); // Run before...

        let (entities, hierarchy, mut parents) = world.system_data::<SystemData>();

        let mut removed = BitSet::new();
        let mut stale = false;

        for event in hierarchy.changed().read(&mut self.reader_id) {
            if let HierarchyEvent::Removed(entity) = event {
                removed.add(entity.id());
                stale = true;
            }
        }

        (&parents, &entities).join()
            .filter(|(parent, _)| removed.contains(parent.id()))
            .map(|(_, child)| child).collect::<Vec<_>>().iter()
            .for_each(|child| {
                let p = parents.remove(*child).unwrap();
                parents.insert(*child, p).unwrap();
            });

        drop(entities); drop(hierarchy); drop(parents);

        if stale {
            self.hierarchy_system.run_now(world); // ...and after
        }
    }
}
