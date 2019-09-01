use web_sys::WebGlUniformLocation;
use std::collections::HashMap;
use super::ShaderConfig;

pub struct ShaderLocations {
    pub map: LocationMap,
}

pub struct ShaderLocation {
    pub a_position: AttributeLocation,
    pub a_color: AttributeLocation,

    pub u_world_view_projection: UniformLocation,
}

pub type LocationMap = HashMap<ShaderConfig, ShaderLocation>;

pub type UniformLocation = WebGlUniformLocation;
pub type AttributeLocation = u32;

pub type Attributes = VecStr;
pub type Uniforms = VecStr;

type VecStr = Vec<&'static str>;

unsafe impl Send for ShaderLocations {}
unsafe impl Sync for ShaderLocations {}
