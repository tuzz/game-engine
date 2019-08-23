use web_sys::Window as Win;
use web_sys::WebGlRenderingContext as Context;

pub struct Window {
    pub window: Win,
    pub context: Context,
}

unsafe impl Send for Window {}
unsafe impl Sync for Window {}
