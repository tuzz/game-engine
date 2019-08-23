use web_sys::Window as Win;
use web_sys::WebGlRenderingContext as Context;

pub struct Window {
    pub window: Win,
    pub context: Context,
}

impl Window {
    pub fn new(window: Win, context: Context) -> Self {
        Self { window, context }
    }
}

unsafe impl Send for Window {}
unsafe impl Sync for Window {}
