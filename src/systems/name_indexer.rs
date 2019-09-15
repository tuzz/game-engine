use specs::prelude::*;
use crate::resources::*;
use crate::components::*;

#[derive(Default)]
pub struct NameIndexer {
    name_reader: Option<ReaderId<ComponentEvent>>,
}

impl<'a> System<'a> for NameIndexer {
    type SystemData = (
        Entities<'a>,
        Write<'a, NameIndex>,
        ReadStorage<'a, Name>,
    );

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        self.name_reader = Some(
            WriteStorage::<Name>::fetch(world).register_reader()
        );
    }

    fn run(&mut self, (entities, mut name_index, names): Self::SystemData) {
        let reader_id = self.name_reader.as_mut().unwrap();

        let mut dirty = BitSet::new();

        for event in names.channel().read(reader_id) {
            match event {
                ComponentEvent::Inserted(id) => dirty.add(*id),
                ComponentEvent::Modified(id) => dirty.add(*id),
                ComponentEvent::Removed(id)  => dirty.add(*id),
            };
        }

        for (_, entity) in (&dirty, &entities).join() {
            match names.get(entity) {
                Some(name) => name_index.insert(&name, &entity),
                None => name_index.remove(&entity),
            }
        }
    }
}
