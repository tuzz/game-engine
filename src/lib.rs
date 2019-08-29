#[macro_use]
extern crate specs_derive;

#[macro_use]
extern crate shred_derive;

#[macro_use]
extern crate impl_ops;

#[macro_use] #[cfg(test)]
extern crate assert_approx_eq;

mod components;
mod resources;
mod systems;
mod utilities;

use specs::prelude::*;
use wasm_bindgen::prelude::*;

use utilities::GameLoop;
use utilities::Matrix4f;
use utilities::Vector3f;
use resources::*;
use components::*;
use systems::*;

#[wasm_bindgen(start)]
pub fn main() {
    let mut game_loop = GameLoop::new();

    let mut webpage = Webpage;
    let mut webgl_shader = WebGlShader;
    let mut webgl_program = WebGlProgram;
    let mut webgl_buffer = WebGlBuffer;
    let mut webgl_render = WebGlRender;
    let mut hierarchy = Hierarchy::new(&mut game_loop.world);
    let mut scene_graph = SceneGraph::default();
    let mut animation = Animation;
    let mut keyboard_input = KeyboardInput;

    game_loop.before(|world| {
        System::setup(&mut webpage, world);
        System::setup(&mut webgl_shader, world);
        System::setup(&mut webgl_program, world);
        System::setup(&mut webgl_buffer, world);
        System::setup(&mut webgl_render, world);
        System::setup(&mut scene_graph, world);
        System::setup(&mut animation, world);
        System::setup(&mut keyboard_input, world);

        let geometry_model = world.create_entity().with(BufferData(vec![
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
        ])).with(Dimensions(3)).build();

        let coloring_model = world.create_entity().with(BufferData(vec![
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
        ])).with(Dimensions(3)).build();

        let cube = world.create_entity()
            .with(Geometry { model: geometry_model })
            .with(Coloring { model: coloring_model })
            .with(LocalTransform(Matrix4f::translation(0., 0., -4.)))
            .build();

        let canvas = world.fetch::<HtmlCanvas>();
        let viewport = Viewport::new(0, 0, canvas.width() , canvas.height());
        //let viewport2 = Viewport::new(canvas.width() / 2, 0, canvas.width() / 2, canvas.height());
        drop(canvas);

        world.create_entity()
            .with(Camera)
            .with(ProjectionTransform(
                Matrix4f::perspective(std::f32::consts::PI / 2., 16. / 9., 0.1, 100.0)
            )).with(LocalTransform(
                Matrix4f::look_at(
                    &Vector3f::new(0., 0., 0.),
                    &Vector3f::new(0.5, 0.1, -1.),
                    &Vector3f::new(0., 1., 0.),
                )
            )).with(viewport).with(ClearColor::white())
            .build();

//        world.create_entity()
//            .with(Camera)
//            .with(ProjectionTransform(
//                Matrix4f::orthographic(-5., 5., -5., 5., -5., 5.)
//            )).with(LocalTransform(
//                Matrix4f::look_at(
//                    &Vector3f::new(0., 0., 0.),
//                    &Vector3f::new(0., 0., -1.),
//                    &Vector3f::new(0., 1., 0.),
//                )
//            )).with(viewport2).with(ClearColor::black())
//            .build();

        let cube2 = world.create_entity()
            .with(Geometry { model: geometry_model })
            .with(Coloring { model: coloring_model })
            .with(SceneParent(cube))
            .with(LocalTransform(
                Matrix4f::scaling(0.5, 0.5, 0.5).translate(4., 0., 0.)
            ))
            .build();

        let mut prev =world.create_entity()
            .with(Geometry { model: geometry_model })
            .with(Coloring { model: coloring_model })
            .with(SceneParent(cube2))
            .with(LocalTransform(
                Matrix4f::scaling(0.5, 0.5, 0.5).translate(0., 4., 0.)
            ))
            .build();

        for i in 1..200 {
            prev = world.create_entity()
                .with(Geometry { model: geometry_model })
                .with(Coloring { model: coloring_model })
                .with(SceneParent(prev))
                .with(LocalTransform(
                    Matrix4f::identity().scale(0.95, 0.95, 0.95).translate(i as f32 * -0.01, 5., 1.)
                ))
                .build();
        }
    });

    game_loop.run(move |_world| {
        // update
    }, move |world| {
        webgl_buffer.run_now(world);
        animation.run_now(world);
        hierarchy.run_now(world);
        scene_graph.run_now(world);
        webgl_render.run_now(world);
    });
}
