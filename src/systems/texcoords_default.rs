use specs::prelude::*;
use crate::components::*;
use crate::utilities::*;

pub struct TexcoordsDefault;

impl<'a> System<'a> for TexcoordsDefault {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Geometry>,
        WriteStorage<'a, TexCoords>,
        WriteStorage<'a, BufferData>,
        WriteStorage<'a, Dimensions>,
    );

    fn run(&mut self, (entities, geometries, mut texcoords, mut buffer_datas, mut dimensions): Self::SystemData) {
        let without_texcoords = (&entities, &geometries, !&texcoords)
            .join().map(|(e, g, _)| (e, g)).collect::<Vec<_>>();

        let mut texcoords_to_add = vec![];
        let mut dimensions_to_add = vec![];

        for (entity, geometry) in without_texcoords {
            match texcoords.get(geometry.model) {
                None => {
                    let geometry_data = buffer_datas.get(geometry.model).unwrap();
                    let texcoords = create_blank_texcoords(geometry_data.iter().count());

                    let texcoords_model = entities.create();

                    buffer_datas.insert(texcoords_model, BufferData(texcoords)).unwrap();
                    dimensions_to_add.push((texcoords_model, Dimensions(2)));

                    texcoords_to_add.push((geometry.model, TexCoords { model: texcoords_model }));
                    texcoords_to_add.push((entity, TexCoords { model: texcoords_model }));
                },
                Some(t) => texcoords_to_add.push((entity, t.clone())),
            };
        }

        for (entity, c) in texcoords_to_add {
            texcoords.insert(entity, c).unwrap();
        }

        for (entity, d) in dimensions_to_add {
            dimensions.insert(entity, d).unwrap();
        }
    }
}
