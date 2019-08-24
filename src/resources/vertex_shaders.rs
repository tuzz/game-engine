use web_sys::WebGlShader;

pub struct VertexShaders {
    pub default: VertexShader,
}

unsafe impl Send for VertexShaders {}
unsafe impl Sync for VertexShaders {}

pub struct VertexShader {
    pub compiled: WebGlShader,
    pub attributes: Attributes,
    pub uniforms: Uniforms,
}

pub type Attributes = VecStr;
pub type Uniforms = VecStr;

type VecStr = Vec<&'static str>;
