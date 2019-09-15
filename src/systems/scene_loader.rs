use specs::prelude::*;
use crate::resources::*;
use crate::components::*;
use crate::utilities::*;

#[derive(Default)]
pub struct SceneLoader {
    loading: bool,
    loaded: bool,
}

#[derive(SystemData)]
pub struct SysData<'a> {
    entities: Entities<'a>,
    html_canvas: ReadExpect<'a, HtmlCanvas>,
    name_index: Read<'a, NameIndex>,

    buffer_datas: WriteStorage<'a, BufferData>,
    dimensions: WriteStorage<'a, Dimensions>,
    images_to_load: WriteStorage<'a, ImageToLoad>,
    geometries: WriteStorage<'a, Geometry>,
    colorings: WriteStorage<'a, Coloring>,
    materials: WriteStorage<'a, Material>,
    textures: WriteStorage<'a, Texture>,

    scene_parents: WriteStorage<'a, SceneParent>,
    local_transforms: WriteStorage<'a, LocalTransform>,
    projection_transforms: WriteStorage<'a, ProjectionTransform>,
    cameras: WriteStorage<'a, Camera>,
    viewports: WriteStorage<'a, Viewport>,
    clear_colors: WriteStorage<'a, ClearColor>,

    directional_lights: WriteStorage<'a, DirectionalLight>,
    point_lights: WriteStorage<'a, PointLight>,

    models_to_load: WriteStorage<'a, ModelsToLoad>,
}

impl<'a> System<'a> for SceneLoader {
    type SystemData = SysData<'a>;

    fn run(&mut self, mut s: SysData) {
        if self.loaded {
            return;
        }

        if self.loading {
            for _ in s.models_to_load.join() {
                return;
            }

            let cube = s.entities.create();
            let geometry_model = *s.name_index.get("geometry_cube").unwrap();
            s.geometries.insert(cube, Geometry { model: geometry_model }).unwrap();
            s.local_transforms.insert(cube, LocalTransform(Matrix4f::scaling(0.1, 0.1, 0.1).translate(0., 0., -4.))).unwrap();

            let camera = s.entities.create();
            let viewport = Viewport::new(0, 0, s.html_canvas.width() , s.html_canvas.height());
            s.cameras.insert(camera, Camera).unwrap();
            s.viewports.insert(camera, viewport).unwrap();
            s.clear_colors.insert(camera, ClearColor::black()).unwrap();
            s.projection_transforms.insert(camera, ProjectionTransform(
                Matrix4f::perspective(std::f32::consts::PI / 2., 16. / 9., 0.1, 100.0)
            )).unwrap();
            s.local_transforms.insert(camera, LocalTransform(
                Matrix4f::look_at(
                    &Vector3f::new(0., 0., 0.),
                    &Vector3f::new(0.5, 0.1, -1.),
                    &Vector3f::new(0., 1., 0.),
                )
            )).unwrap();

            let directional = s.entities.create();
            s.directional_lights.insert(directional, DirectionalLight).unwrap();
            s.local_transforms.insert(directional, LocalTransform (
                Matrix4f::translation(0., 1., 0.)
            )).unwrap();

            let point = s.entities.create();
            s.point_lights.insert(point, PointLight).unwrap();
            s.local_transforms.insert(point, LocalTransform(
                Matrix4f::translation(0., 0., -2.)
            )).unwrap();

            self.loading = false;
            self.loaded = true;
        } else {
            let model_loader = s.entities.create();

            s.models_to_load.insert(model_loader, ModelsToLoad::new(&[
                "assets/objects/cube.obj",
            ], &[
                "assets/materials/default.mtl",
            ])).unwrap();

            self.loading = true;
        }
    }
}
