use specs::prelude::*;
use web_sys::WebGlRenderingContext as GL;

use super::resources::ClearColor;
use super::resources::Window;

pub struct Viewport;

impl<'a> System<'a> for Viewport {
    type SystemData = (Read<'a, ClearColor>, ReadExpect<'a, Window>);

    fn run(&mut self, (clear_color, window): Self::SystemData) {
        let ClearColor(r, g, b, a) = *clear_color;

        let canvas = &window.canvas;
        let context = &window.context;

        context.viewport(0, 0, canvas.width() as i32, canvas.height() as i32);

        context.enable(GL::BLEND);
        context.blend_func(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);

        context.enable(GL::CULL_FACE);
        context.enable(GL::DEPTH_TEST);

        context.clear_color(r, g, b, a);
        context.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
    }
}
