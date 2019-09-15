use specs::prelude::*;
use crate::resources::*;
use crate::components::*;
use crate::utilities::*;

#[derive(Default)]
pub struct SceneLoader {
    loaded: bool,
}

#[derive(SystemData)]
pub struct SysData<'a> {
    entities: Entities<'a>,
    html_canvas: ReadExpect<'a, HtmlCanvas>,

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

        let geometry_model = s.entities.create();

        s.buffer_datas.insert(geometry_model, BufferData(vec![
            // Front
            -1., -1.,  1.,
             1., -1.,  1.,
             1.,  1.,  1.,

             1.,  1.,  1.,
            -1.,  1.,  1.,
            -1., -1.,  1.,

            // Back
            -1., -1., -1.,
             1.,  1., -1.,
             1., -1., -1.,

             1.,  1., -1.,
            -1., -1., -1.,
            -1.,  1., -1.,

            // Left
            -1., -1.,  1.,
            -1.,  1.,  1.,
            -1., -1., -1.,

            -1., -1., -1.,
            -1.,  1.,  1.,
            -1.,  1., -1.,

            // Right
             1., -1.,  1.,
             1., -1., -1.,
             1.,  1., -1.,

             1.,  1., -1.,
             1.,  1.,  1.,
             1., -1.,  1.,

            // Bottom
            -1., -1.,  1.,
            -1., -1., -1.,
             1., -1.,  1.,

             1., -1.,  1.,
            -1., -1., -1.,
             1., -1., -1.,

            // Top
            -1.,  1.,  1.,
             1.,  1.,  1.,
             1.,  1., -1.,

             1.,  1., -1.,
            -1.,  1., -1.,
            -1.,  1.,  1.,
        ])).unwrap();

        s.dimensions.insert(geometry_model, Dimensions(3)).unwrap();

        let coloring_model = s.entities.create();

        s.buffer_datas.insert(coloring_model, BufferData(vec![
            // Front
            0.5, 0.5, 0.5,
            0.5, 0.5, 0.5,
            0.5, 0.5, 0.5,

            0., 0., 1.,
            0., 0., 1.,
            0., 0., 1.,

            // Back
            0., 1., 0.,
            0., 1., 0.,
            0., 1., 0.,

            0., 1., 1.,
            0., 1., 1.,
            0., 1., 1.,

            // Left
            1., 0., 0.,
            1., 0., 0.,
            1., 0., 0.,

            1., 0., 1.,
            1., 0., 1.,
            1., 0., 1.,

            // Right
            1., 1., 0.,
            1., 1., 0.,
            1., 1., 0.,

            0., 0., 0.5,
            0., 0., 0.5,
            0., 0., 0.5,

            // Bottom
            0., 0.5, 0.,
            0., 0.5, 0.,
            0., 0.5, 0.,

            0.5, 0., 0.,
            0.5, 0., 0.,
            0.5, 0., 0.,

            // Top
            0.5, 0., 1.,
            0.5, 0., 1.,
            0.5, 0., 1.,

            1., 0.5, 0.,
            1., 0.5, 0.,
            1., 0.5, 0.,
        ])).unwrap();

        s.dimensions.insert(coloring_model, Dimensions(3)).unwrap();

        let texture_model = s.entities.create();

        s.buffer_datas.insert(texture_model, BufferData(vec![
            // Front
            0., 0.,
            1., 0.,
            1., 1.,

            1., 1.,
            0., 1.,
            0., 0.,

            // Back
            1., 0.,
            0., 1.,
            0., 0.,

            0., 1.,
            1., 0.,
            1., 1.,

            // Left
            1., 0.,
            1., 1.,
            0., 0.,

            0., 0.,
            1., 1.,
            0., 1.,

            // Right
            0., 0.,
            1., 0.,
            1., 1.,

            1., 1.,
            0., 1.,
            0., 0.,

            // Bottom
            0., 1.,
            0., 0.,
            1., 1.,

            1., 1.,
            0., 0.,
            1., 0.,

            // Top
            0., 0.,
            1., 0.,
            1., 1.,

            1., 1.,
            0., 1.,
            0., 0.,
        ])).unwrap();

        s.dimensions.insert(texture_model, Dimensions(2)).unwrap();

        s.images_to_load.insert(texture_model, ImageToLoad::new("assets/textures/cc-logo.png")).unwrap();

        let cube = s.entities.create();
        s.geometries.insert(cube, Geometry { model: geometry_model }).unwrap();
        s.materials.insert(cube, Material::gold()).unwrap();
        s.textures.insert(cube, Texture { model: texture_model }).unwrap();
        s.local_transforms.insert(cube, LocalTransform(Matrix4f::translation(0., 0., -4.))).unwrap();

        let viewport = Viewport::new(0, 0, s.html_canvas.width() , s.html_canvas.height());

        let camera = s.entities.create();
        s.cameras.insert(camera, Camera).unwrap();
        s.viewports.insert(camera, viewport).unwrap();

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

        s.clear_colors.insert(camera, ClearColor::black()).unwrap();

        let cube2 = s.entities.create();

        s.geometries.insert(cube2, Geometry { model: geometry_model }).unwrap();
        s.colorings.insert(cube2, Coloring { model: coloring_model }).unwrap();
        s.textures.insert(cube2, Texture { model: texture_model }).unwrap();
        s.scene_parents.insert(cube2, SceneParent(cube)).unwrap();

        s.local_transforms.insert(cube2, LocalTransform(
            Matrix4f::scaling(0.5, 0.5, 0.5).translate(4., 0., 0.)
        )).unwrap();

        let mut prev = s.entities.create();
        s.geometries.insert(prev, Geometry { model: geometry_model }).unwrap();
        s.textures.insert(prev, Texture { model: texture_model }).unwrap();
        s.scene_parents.insert(prev, SceneParent(cube2)).unwrap();

        s.local_transforms.insert(prev, LocalTransform(
            Matrix4f::scaling(0.5, 0.5, 0.5).translate(0., 4., 0.)
        )).unwrap();

        for i in 1..200 {
            let next = s.entities.create();

            s.geometries.insert(next, Geometry { model: geometry_model }).unwrap();
            s.colorings.insert(next, Coloring { model: coloring_model }).unwrap();
            s.scene_parents.insert(next, SceneParent(prev)).unwrap();

            s.local_transforms.insert(next, LocalTransform(
                Matrix4f::identity().scale(0.95, 0.95, 0.95).translate(i as f32 * -0.01, 5., 1.)
            )).unwrap();

            prev = next;
        }

        let directional = s.entities.create();
        s.directional_lights.insert(directional, DirectionalLight).unwrap();
        s.local_transforms.insert(directional, LocalTransform (
            Matrix4f::translation(0., 1., 0.)
        )).unwrap();

        let point = s.entities.create();
        s.point_lights.insert(point, PointLight).unwrap();
        s.local_transforms.insert(point, LocalTransform (
            Matrix4f::translation(0., 0., -2.)
        )).unwrap();

        let model_loader = s.entities.create();
        s.models_to_load.insert(model_loader, ModelsToLoad::new(&[
            "assets/objects/cornell_box.obj",
        ], &[
            "assets/materials/cornell_box.mtl",
            "assets/materials/cornell_box2.mtl",
        ])).unwrap();

        self.loaded = true;
    }
}
