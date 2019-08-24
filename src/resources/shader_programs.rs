use web_sys::WebGlProgram;

pub struct ShaderPrograms {
    pub default: WebGlProgram,
}

unsafe impl Send for ShaderPrograms {}
unsafe impl Sync for ShaderPrograms {}
