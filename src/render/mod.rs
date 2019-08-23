use specs::prelude::*;
use wasm_bindgen::prelude::*;
use super::resources::Window;

use web_sys::WebGlRenderingContext as GL;

mod buffer;
mod feed;
mod shader;
mod program;
mod viewport;

use program::Program;
use buffer::Buffer;
use viewport::Viewport;
use feed::*;

pub struct Render;

impl<'a> System<'a> for Render {
    type SystemData = ();

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        let window = window_resource(world);
        let canvas = &window.canvas;
        let context = &window.context;

        let program = Program::default(context);

        let position_buffer = Buffer::new(context, &[
            0.0, 0.0,
            0.0, 0.5,
            0.7, 0.0,
        ]);

        let color_buffer = Buffer::new(context, &[
            1.0, 0.5, 0.5, 1.0,
            0.5, 1.0, 0.5, 1.0,
            0.5, 0.5, 1.0, 1.0,
        ]);

        let viewport = Viewport::new(0.0, 0.0, 0.0, 0.0);
        viewport.clear(canvas, context);

        program.enable(context);

        feed_attribute(context, &program, "a_position", &position_buffer, 2);
        feed_attribute(context, &program, "a_color", &color_buffer, 4);

        feed_uniform(context, &program, "u_matrix", &[
            1.0, 0.0, 0.0,
            0.0, 1.0, 0.0,
            0.0, 0.0, 1.0,
        ]);

        context.draw_arrays(GL::TRIANGLES, 0, position_buffer.len(2));
    }

    fn run(&mut self, (): Self::SystemData) {
        log("Hello, console!");
    }
}

fn window_resource(world: &mut World) -> &mut Window {
    world.get_mut::<Window>().unwrap()
}

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
