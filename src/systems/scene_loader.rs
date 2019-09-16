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
    geometry_groups: WriteStorage<'a, GeometryGroup>,
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

            let skull = s.entities.create();
            s.geometry_groups.insert(skull, GeometryGroup::new("assets/objects/skull.obj")).unwrap();
            s.local_transforms.insert(skull, LocalTransform(
                Matrix4f::translation(0., -10., -30.).x_rotate(-PI / 2.)
            )).unwrap();

            let left_cube = s.entities.create();
            s.geometries.insert(left_cube, Geometry::find(&s.name_index, "cube")).unwrap();
            s.materials.insert(left_cube, Material::find(&s.name_index, "gold")).unwrap();
            s.local_transforms.insert(left_cube, LocalTransform(
                Matrix4f::translation(-37., 17., -30.).scale(5., 5., 5.)
            )).unwrap();

            let right_cube = s.entities.create();
            s.geometries.insert(right_cube, Geometry::find(&s.name_index, "cube")).unwrap();
            s.textures.insert(right_cube, Texture::find(&s.name_index, "assets/textures/tuzz.jpg")).unwrap();
            s.local_transforms.insert(right_cube, LocalTransform(
                Matrix4f::translation(37., 17., -30.).scale(5., 5., 5.)
            )).unwrap();

            for i in 0..20 {
                let ratio = i as f32 / 20.0;
                let mini_skull = s.entities.create();

                s.geometry_groups.insert(mini_skull, GeometryGroup::new("assets/objects/skull.obj")).unwrap();
                s.scene_parents.insert(mini_skull, SceneParent(skull)).unwrap();
                s.local_transforms.insert(mini_skull, LocalTransform(
                    Matrix4f::translation(0., 0., 5.)
                        .z_rotate(2. * PI * ratio)
                        .translate(20., 0., 0.)
                        .z_rotate(PI / 2.)
                        .scale(0.1, 0.1, 0.1)
                )).unwrap();

                let center = s.entities.create();
                s.scene_parents.insert(center, SceneParent(mini_skull)).unwrap();
                s.local_transforms.insert(center, LocalTransform(
                    Matrix4f::translation(0., 0., 8.).scale(4., 4., 4.))
                ).unwrap();

                for j in 0..5 {
                    let jratio = j as f32 / 5.0;
                    let cube = s.entities.create();

                    s.geometries.insert(cube, Geometry::find(&s.name_index, "cube")).unwrap();
                    s.scene_parents.insert(cube, SceneParent(center)).unwrap();
                    s.local_transforms.insert(cube, LocalTransform(
                        Matrix4f::translation(8., 8., 0.)
                            .y_rotate(2. * PI / jratio)
                            .translate(5., 0., 5.)
                    )).unwrap();
                }
            }

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
                    &Vector3f::new(0., 0., -1.),
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
                Matrix4f::translation(0., 1., -50.)
            )).unwrap();

            self.loading = false;
            self.loaded = true;
        } else {
            let model_loader = s.entities.create();

            s.models_to_load.insert(model_loader, ModelsToLoad::new(&[
                "assets/objects/skull.obj",
                "assets/objects/cube.obj",
            ], &[
                "assets/materials/skull.mtl",
                "assets/materials/default.mtl",
            ])).unwrap();

            self.loading = true;
        }
    }
}
