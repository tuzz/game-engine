use specs::prelude::*;
use super::resources::Window;

use web_sys::WebGlRenderingContext as GL;

mod buffer;
mod feed;
mod shader;
mod program;

use program::Program;
use buffer::Buffer;
use feed::*;

#[derive(Default)]
pub struct Render {
    program: Option<Program>,

    positions: Option<Buffer>,
    colors: Option<Buffer>,

    offset: f32,
}

impl<'a> System<'a> for Render {
    type SystemData = ReadExpect<'a, Window>;

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        let window = window_resource(world);
        let context = &window.context;

        self.program = Some(Program::default(context));

        self.positions = Some(Buffer::new(context, &[
            -1.0, 0.0,
            -0.5, 0.0,
            -1.0, 0.5,
        ]));

        self.colors = Some(Buffer::new(context, &[
            1.0, 0.5, 0.5, 1.0,
            0.5, 1.0, 0.5, 1.0,
            0.5, 0.5, 1.0, 1.0,
        ]));
    }

    fn run(&mut self, window: Self::SystemData) {
        let context = &window.context;

        let program = self.program.as_ref().unwrap();
        let positions = self.positions.as_ref().unwrap();
        let colors = self.colors.as_ref().unwrap();

        program.enable(context);

        feed_attribute(context, program, "a_position", &positions, 2);
        feed_attribute(context, program, "a_color", &colors, 4);

        feed_uniform(context, &program, "u_matrix", &[
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            self.offset / 2.0, 0.0, 0.0, 1.0,
        ]);

        context.draw_arrays(GL::TRIANGLES, 0, positions.len(2));

        feed_uniform(context, &program, "u_matrix", &[
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            self.offset, 0.0, 0.5, 1.0,
        ]);

        context.draw_arrays(GL::TRIANGLES, 0, positions.len(2));

        self.offset += 0.002;

        if self.offset > 1.5 {
            self.offset = 0.0;
        }
    }
}

fn window_resource(world: &mut World) -> &mut Window {
    world.get_mut::<Window>().unwrap()
}
