use specs::prelude::*;
use crate::components::*;
use crate::utilities::Vector3f;
use std::collections::VecDeque;
use std::iter::once;

pub struct VertexNormals;

impl<'a> System<'a> for VertexNormals {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Geometry>,

        WriteStorage<'a, Normals>,
        WriteStorage<'a, BufferData>,
    );

    fn run(&mut self, (entities, geometries, mut normals, mut buffer_datas): Self::SystemData) {
        let mut normals_to_add = VecDeque::new();

        for (geometry, (), entity) in (&geometries, !&normals, &entities).join() {
            match normals.get(geometry.model) {
                None => {
                    let vertices = buffer_datas.get(geometry.model).unwrap();
                    let vertex_normals = vertex_normals(vertices);
                    let model = entities.create();

                    buffer_datas.insert(model, vertex_normals).unwrap();
                    normals_to_add.push_front((geometry.model, Normals::new(model)));
                },
                Some(n) => { normals_to_add.push_back((entity, n.clone())); },
            }
        }

        for (entity, n) in normals_to_add {
            normals.insert(entity, n).unwrap();
        }
    }
}

fn vertex_normals(buffer_data: &BufferData) -> BufferData {
    BufferData(buffer_data.chunks(9).flat_map(|chunk| {
        let point1 = (chunk[0], chunk[1], chunk[2]).into();
        let point2 = (chunk[3], chunk[4], chunk[5]).into();
        let point3 = (chunk[6], chunk[7], chunk[8]).into();

        let normal = Vector3f::surface_normal(point1, point2, point3);
        let (x, y, z) = (once(normal.x), once(normal.y), once(normal.z));

        x.chain(y).chain(z).cycle().take(9)
    }).collect::<Vec<_>>())
}
