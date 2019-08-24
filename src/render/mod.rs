use specs::prelude::*;
use web_sys::WebGlRenderingContext as GL;
use crate::resources::{*, WebGlContext};

mod buffer;
mod feed;

use buffer::Buffer;
use feed::*;

#[derive(Default)]
pub struct Render {
    positions: Option<Buffer>,
    colors: Option<Buffer>,

    offset: f32,
}

impl<'a> System<'a> for Render {
    type SystemData = (
        ReadExpect<'a, WebGlContext>,
        ReadExpect<'a, ShaderPrograms>,
    );

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        let context = world.get_mut::<WebGlContext>().unwrap();

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

    fn run(&mut self, (context, programs): Self::SystemData) {
        let program = &programs.default;
        let positions = self.positions.as_ref().unwrap();
        let colors = self.colors.as_ref().unwrap();

        context.use_program(Some(&program.compiled));

        feed_attribute(&context, program, "a_position", &positions, 2);
        feed_attribute(&context, program, "a_color", &colors, 4);

        feed_uniform(&context, program, "u_matrix", &[
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            self.offset / 2.0, 0.0, 0.0, 1.0,
        ]);

        context.draw_arrays(GL::TRIANGLES, 0, positions.len(2));

        feed_uniform(&context, &program, "u_matrix", &[
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
