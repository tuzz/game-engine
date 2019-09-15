use specs::prelude::*;
use crate::resources::*;
use crate::components::*;
use crate::utilities::*;

pub struct GroupExpander;

#[derive(SystemData)]
pub struct SysData<'a> {
    entities: Entities<'a>,
    model_groups: ReadExpect<'a, ModelGroups>,
    geometry_groups: WriteStorage<'a, GeometryGroup>,
    scene_parents: WriteStorage<'a, SceneParent>,
    geometries: WriteStorage<'a, Geometry>,
    local_transforms: WriteStorage<'a, LocalTransform>,
}

impl<'a> System<'a> for GroupExpander {
    type SystemData = SysData<'a>;

    fn run(&mut self, mut s: SysData) {
        let mut expanded = vec![];

        for (parent, name) in (&s.entities, &s.geometry_groups).join() {
            let bitset = s.model_groups.get(&name.0).unwrap();

            for (_, geometry_model) in (bitset, &s.entities).join() {
                let child = s.entities.create();

                s.scene_parents.insert(child, SceneParent(parent)).unwrap();
                s.geometries.insert(child, Geometry { model: geometry_model }).unwrap();
                s.local_transforms.insert(child, LocalTransform(Matrix4f::identity())).unwrap();
            }

            expanded.push(parent);
        }

        for entity in expanded {
            s.geometry_groups.remove(entity).unwrap();
        }
    }
}
