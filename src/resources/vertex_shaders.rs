use web_sys::WebGlShader;

pub struct VertexShaders {
    pub default: WebGlShader,
}

unsafe impl Send for VertexShaders {}
unsafe impl Sync for VertexShaders {}
