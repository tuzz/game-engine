use specs::prelude::*;
use crate::components::*;

pub struct MaterialDefault;

#[derive(SystemData)]
pub struct SysData<'a> {
    pub entities: Entities<'a>,
    pub geometries: ReadStorage<'a, Geometry>,

    pub materials: WriteStorage<'a, Material>,
    pub ambients: WriteStorage<'a, Ambient>,
    pub diffuses: WriteStorage<'a, Diffuse>,
    pub speculars: WriteStorage<'a, Specular>,
    pub shinies: WriteStorage<'a, Shininess>,
}

impl<'a> System<'a> for MaterialDefault {
    type SystemData = SysData<'a>;

    fn run(&mut self, mut s: SysData) {
        let without_materials = (&s.entities, &s.geometries, !&s.materials)
            .join().map(|(e, g, _)| (e, g)).collect::<Vec<_>>();

        let mut materials_to_add = vec![];

        for (entity, geometry) in without_materials {
            match s.materials.get(geometry.model) {
                None => {
                    let material_model = s.entities.create();

                    s.ambients.insert(material_model, Ambient::default()).unwrap();
                    s.diffuses.insert(material_model, Diffuse::default()).unwrap();
                    s.speculars.insert(material_model, Specular::default()).unwrap();
                    s.shinies.insert(material_model, Shininess::default()).unwrap();

                    materials_to_add.push((geometry.model, Material { model: material_model }));
                    materials_to_add.push((entity, Material { model: material_model }));
                },
                Some(m) => materials_to_add.push((entity, m.clone())),
            };
        }

        for (entity, c) in materials_to_add {
            s.materials.insert(entity, c).unwrap();
        }
    }
}
