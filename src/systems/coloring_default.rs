use specs::prelude::*;
use crate::components::*;

pub struct ColoringDefault;

impl<'a> System<'a> for ColoringDefault {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Geometry>,
        WriteStorage<'a, Coloring>,
        WriteStorage<'a, BufferData>,
        WriteStorage<'a, Dimensions>,
    );

    fn run(&mut self, (entities, geometries, mut colorings, mut buffer_datas, mut dimensions): Self::SystemData) {
        let without_colorings = (&entities, &geometries, !&colorings)
            .join().map(|(e, g, _)| (e, g)).collect::<Vec<_>>();

        let mut colorings_to_add = vec![];
        let mut dimensions_to_add = vec![];

        for (entity, geometry) in without_colorings {
            match colorings.get(geometry.model) {
                None => {
                    let geometry_data = buffer_datas.get(geometry.model).unwrap();
                    let coloring_data = color_every_vertex_white(geometry_data);
                    let geo_dimensions = dimensions.get(geometry.model).unwrap();

                    let coloring_model = entities.create();
                    buffer_datas.insert(coloring_model, BufferData(coloring_data)).unwrap();
                    dimensions_to_add.push((coloring_model, geo_dimensions.clone()));

                    colorings_to_add.push((geometry.model, Coloring { model: coloring_model }));
                    colorings_to_add.push((entity, Coloring { model: coloring_model }));
                },
                Some(c) => colorings_to_add.push((entity, c.clone())),
            };
        }

        for (entity, c) in colorings_to_add {
            colorings.insert(entity, c).unwrap();
        }

        for (entity, d) in dimensions_to_add {
            dimensions.insert(entity, d).unwrap();
        }
    }
}

fn color_every_vertex_white(geometry_data: &BufferData) -> Vec<f32> {
    geometry_data.iter().map(|_| 1.0).collect::<Vec<_>>()
}
