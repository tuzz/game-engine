use web_sys::WebGlRenderingContext as GL;
use std::ops::Deref;

pub struct WebGlContext(pub GL);

impl Deref for WebGlContext {
    type Target = GL;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

unsafe impl Send for WebGlContext {}
unsafe impl Sync for WebGlContext {}
