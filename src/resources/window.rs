use std::ops::Deref;

use web_sys::Window as Win;
use web_sys::HtmlCanvasElement as Canvas;
use web_sys::WebGlRenderingContext as Context;

pub struct Window {
    pub window: Win,
    pub canvas: Canvas,
    pub context: Context,
}

impl Window {
    pub fn new(window: Win, canvas: Canvas, context: Context) -> Self {
        Self { window, canvas, context }
    }
}

impl Deref for Window {
    type Target = Win;

    fn deref(&self) -> &Self::Target {
        &self.window
    }
}

unsafe impl Send for Window {}
unsafe impl Sync for Window {}
