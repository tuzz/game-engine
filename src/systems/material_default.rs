use specs::prelude::*;
use crate::components::*;

pub struct MaterialDefault;

impl<'a> System<'a> for MaterialDefault {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Geometry>,
        WriteStorage<'a, Material>,
    );

    fn run(&mut self, (entities, geometries, mut materials): Self::SystemData) {
        let without_materials = (&entities, &geometries, !&materials)
            .join().map(|(e, _, _)| e).collect::<Vec<_>>();

        for entity in without_materials {
            materials.insert(entity, Material::default()).unwrap();
        }
    }
}
