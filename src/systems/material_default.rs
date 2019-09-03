use specs::prelude::*;
use crate::components::*;

pub struct MaterialDefault;

impl<'a> System<'a> for MaterialDefault {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Geometry>,
        ReadStorage<'a, WorldTransform>,
        WriteStorage<'a, Material>,
    );

    fn run(&mut self, (entities, geometries, transforms, mut materials): Self::SystemData) {
        let without_materials = (&entities, &geometries, &transforms, !&materials)
            .join().map(|(e, _, _, _)| e).collect::<Vec<_>>();

        for entity in without_materials {
            materials.insert(entity, Material::default()).unwrap();
        }
    }
}
