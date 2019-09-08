use web_sys::WebGlUniformLocation;
use std::collections::HashMap;
use super::ShaderConfig;

pub struct ShaderLocations {
    pub map: LocationMap,
}

pub struct ShaderLocation {
    pub a_position: AttributeLocation,
    pub a_normal: Option<AttributeLocation>,
    pub a_color: AttributeLocation,

    pub a_texcoords: AttributeLocation,
    pub u_texture: UniformLocation,

    pub u_world: Option<UniformLocation>,
    pub u_world_view_projection: UniformLocation,
    pub u_inverse_world: Option<UniformLocation>,
    pub u_camera_position: Option<UniformLocation>,

    pub u_material_ambient: UniformLocation,
    pub u_material_diffuse: UniformLocation,
    pub u_material_specular: UniformLocation,
    pub u_material_shininess: UniformLocation,

    pub u_directional_light_vector: Vec<UniformLocation>,
    pub u_point_light_position: Vec<UniformLocation>,
}

pub type LocationMap = HashMap<ShaderConfig, ShaderLocation>;

pub type UniformLocation = WebGlUniformLocation;
pub type AttributeLocation = u32;

pub type Attributes = VecStr;
pub type Uniforms = VecStr;

type VecStr = Vec<&'static str>;

unsafe impl Send for ShaderLocations {}
unsafe impl Sync for ShaderLocations {}
