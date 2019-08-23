use specs::prelude::*;
use wasm_bindgen::prelude::*;
use super::resources::Window;

use web_sys::WebGlRenderingContext as GL;

mod buffer;
mod shader;
mod program;
mod viewport;

use program::Program;
use buffer::Buffer;
use viewport::Viewport;

pub struct Render;

impl<'a> System<'a> for Render {
    type SystemData = ();

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        let window = window_resource(world);
        let canvas = &window.canvas;
        let context = &window.context;

        let program = Program::default(context);

        let a_position = program.attribute_location("a_position");
        let a_color = program.attribute_location("a_color");
        let u_matrix = program.uniform_location("u_matrix");

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

        context.enable_vertex_attrib_array(a_position as u32);
        position_buffer.bind(context);
        context.vertex_attrib_pointer_with_i32(a_position as u32, 2, GL::FLOAT, false, 0, 0);

        //
        context.enable_vertex_attrib_array(a_color as u32);
        color_buffer.bind(context);
        context.vertex_attrib_pointer_with_i32(a_color as u32, 4, GL::FLOAT, false, 0, 0);
        //

        let matrix = [
            1.0, 0.0, 0.0,
            0.0, 1.0, 0.0,
            0.0, 0.0, 1.0,
        ];
        context.uniform_matrix3fv_with_f32_array(Some(&u_matrix), false, &matrix);
        context.draw_arrays(GL::TRIANGLES, 0, 3);
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
