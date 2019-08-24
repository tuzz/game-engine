use std::collections::HashMap;

use web_sys::WebGlProgram;
pub use web_sys::WebGlUniformLocation as UniformLocation;

pub struct ShaderPrograms {
    pub default: ShaderProgram,
}

unsafe impl Send for ShaderPrograms {}
unsafe impl Sync for ShaderPrograms {}

pub struct ShaderProgram {
    pub compiled: WebGlProgram,
    pub attribute_map: AttributeMap,
    pub uniform_map: UniformMap,
}

pub type AttributeMap = HashMap<&'static str, AttributeLocation>;
pub type UniformMap = HashMap<&'static str, UniformLocation>;

pub type AttributeLocation = u32;

impl ShaderProgram {
    pub fn attribute_location(&self, name: &str) -> AttributeLocation {
        self.attribute_map.get(name).unwrap().to_owned()
    }

    pub fn uniform_location(&self, name: &str) -> UniformLocation {
        self.uniform_map.get(name).unwrap().to_owned()
    }
}
