use std::collections::HashMap;
use web_sys::WebGlProgram;
use super::*;

#[derive(Default)]
pub struct ShaderPrograms {
    pub map: ProgramMap,
}

pub type ProgramMap = HashMap<ShaderConfig, ShaderProgram>;
pub type AttributeMap = HashMap<String, AttributeLocation>;
pub type UniformMap = HashMap<String, UniformLocation>;

pub struct ShaderProgram {
    pub compiled: WebGlProgram,
    pub attribute_map: AttributeMap,
    pub uniform_map: UniformMap,
}

unsafe impl Send for ShaderPrograms {}
unsafe impl Sync for ShaderPrograms {}
