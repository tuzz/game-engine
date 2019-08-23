use web_sys::WebGlRenderingContext as GL;
use web_sys::HtmlCanvasElement as Canvas;

pub struct Viewport {
    clear_color: (f32, f32, f32, f32),
}

impl Viewport {
    pub fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        Self { clear_color: (red, green, blue, alpha) }
    }

    pub fn clear(&self, canvas: &Canvas, context: &GL) {
        let (r, g, b, a) = self.clear_color;

        context.viewport(0, 0, canvas.width() as i32, canvas.height() as i32);
        context.clear_color(r, g, b, a);
        context.clear(GL::COLOR_BUFFER_BIT);

        context.enable(GL::BLEND);
        context.blend_func(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);
    }
}
