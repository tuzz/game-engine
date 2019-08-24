use std::ops::Deref;

use web_sys::Window as Win;
use web_sys::HtmlCanvasElement as Canvas;

pub struct Window {
    pub window: Win,
    pub canvas: Canvas,
}

impl Window {
    pub fn new(window: Win, canvas: Canvas) -> Self {
        Self { window, canvas }
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
