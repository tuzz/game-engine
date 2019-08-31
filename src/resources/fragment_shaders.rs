use web_sys::WebGlShader;
use super::vertex_shaders::Uniforms;

pub struct FragmentShaders {
    pub default: FragmentShader,
}

unsafe impl Send for FragmentShaders {}
unsafe impl Sync for FragmentShaders {}

pub struct FragmentShader {
    pub compiled: WebGlShader,
    pub uniforms: Uniforms,
}
