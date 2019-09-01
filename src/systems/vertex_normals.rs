use specs::prelude::*;
use crate::components::*;
use crate::utilities::Triangle;
use std::collections::VecDeque;
use std::iter::once;

pub struct VertexNormals;

// This system assumes geometries are 3D. It doesn't really make sense for 2D
// anyway since the normals will always point towards the camera: (0, 0, 1)

impl<'a> System<'a> for VertexNormals {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Geometry>,

        WriteStorage<'a, Normals>,
        WriteStorage<'a, BufferData>,
        WriteStorage<'a, Dimensions>,
    );

    fn run(&mut self, (entities, geometries, mut normals, mut buffer_datas, mut dimensions): Self::SystemData) {
        let mut normals_to_add = VecDeque::new();

        for (geometry, (), entity) in (&geometries, !&normals, &entities).join() {
            match normals.get(geometry.model) {
                None => {
                    let vertices = buffer_datas.get(geometry.model).unwrap();
                    let vertex_normals = vertex_normals(vertices);
                    let model = entities.create();

                    buffer_datas.insert(model, vertex_normals).unwrap();
                    dimensions.insert(model, Dimensions(3)).unwrap();

                    normals_to_add.push_front((geometry.model, Normals::new(model)));
                    normals_to_add.push_back((entity, Normals::new(model)));
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
        let p1 = (chunk[0], chunk[1], chunk[2]).into();
        let p2 = (chunk[3], chunk[4], chunk[5]).into();
        let p3 = (chunk[6], chunk[7], chunk[8]).into();

        let normal = Triangle { p1, p2, p3 }.surface_normal();
        let (x, y, z) = (once(normal.x), once(normal.y), once(normal.z));

        x.chain(y).chain(z).cycle().take(9)
    }).collect::<Vec<_>>())
}

#[cfg(test)]
mod test {
    use super::*;

    fn setup() -> (World, VertexNormals) {
        let mut world = World::new();
        let mut system = VertexNormals;

        System::setup(&mut system, &mut world);

        (world, system)
    }

    #[test]
    fn it_creates_an_entity_to_hold_normal_data_and_associates_it_with_model_and_instance() {
        let (mut world, mut system) = setup();

        let vertices = vec![
            0., 0., 0.,
            1., 0., 0.,
            0., 1., 0.,

            0., 0., 0.,
            0., 0., 2.,
            0., 1., 0.,
        ];

        let geometry_model = world.create_entity().with(BufferData(vertices)).build();
        let instance = world.create_entity().with(Geometry { model: geometry_model }).build();

        system.run_now(&mut world);

        let normals = world.read_storage::<Normals>();

        let model_normals = normals.get(geometry_model).unwrap();
        let instance_normals = normals.get(instance).unwrap();

        assert_eq!(model_normals, instance_normals);

        let buffer_datas = world.read_storage::<BufferData>();
        let data = buffer_datas.get(instance_normals.model).unwrap();

        assert_eq!(data.0, &[
            0., 0., 1., // Normal faces in the Z = 1 direction
            0., 0., 1.,
            0., 0., 1.,

           -2., 0., 0., // Normal faces in the X = -1 direction
           -2., 0., 0.,
           -2., 0., 0.,
        ]);
    }
}
