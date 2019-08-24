use web_sys::WebGlShader;

pub struct FragmentShaders {
    pub default: WebGlShader,
}

unsafe impl Send for FragmentShaders {}
unsafe impl Sync for FragmentShaders {}
