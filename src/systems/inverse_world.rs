use specs::prelude::*;
use crate::components::*;

#[derive(Default)]
pub struct InverseWorld {
    world_transform_reader: Option<ReaderId<ComponentEvent>>,
}

impl<'a> System<'a> for InverseWorld {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, WorldTransform>,
        WriteStorage<'a, InverseWorldTransform>,
    );

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        self.world_transform_reader = Some(
            WriteStorage::<WorldTransform>::fetch(world).register_reader()
        );
    }

    fn run(&mut self, (entities, world_transforms, mut inverse_world_transforms): Self::SystemData) {
        let reader_id = self.world_transform_reader.as_mut().unwrap();
        let mut dirty = BitSet::new();

        for event in world_transforms.channel().read(reader_id) {
            match event {
                ComponentEvent::Inserted(id) => dirty.add(*id),
                ComponentEvent::Modified(id) => dirty.add(*id),
                ComponentEvent::Removed(id)  => dirty.add(*id),
            };
        }

        for (_, entity) in (dirty, &entities).join() {
            match world_transforms.get(entity) {
                Some(transform) => {
                    let inverse = InverseWorldTransform(transform.inverse());

                    inverse_world_transforms.insert(entity, inverse).unwrap();
                },
                None => {
                    inverse_world_transforms.remove(entity);
                },
            };
        }
    }
}
