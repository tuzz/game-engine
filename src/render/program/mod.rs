use web_sys::WebGlRenderingContext as GL;
use web_sys::WebGlProgram;
use web_sys::WebGlUniformLocation as UniformLocation;

use std::collections::HashMap;

use super::shader::*;

type AttributeLocation = u32;

type AttributeMap = HashMap<String, AttributeLocation>;
type UniformMap = HashMap<String, UniformLocation>;

pub struct Program {
    pub compiled: WebGlProgram,

    pub vert: VertexShader,
    pub frag: FragmentShader,

    pub attribute_map: AttributeMap,
    pub uniform_map: UniformMap,
}

impl Program {
    pub fn new(context: &GL, vert: VertexShader, frag: FragmentShader) -> Self {
        let program = context.create_program().unwrap();

        context.attach_shader(&program, &vert.compiled);
        context.attach_shader(&program, &frag.compiled);
        context.link_program(&program);

        let attribute_map = attribute_map(&program, context, &vert);
        let uniform_map = uniform_map(&program, context, &vert);

        Self { compiled: program, vert, frag, attribute_map, uniform_map }
    }

    pub fn default(context: &GL) -> Self {
        let vert = VertexShader::default(context);
        let frag = FragmentShader::default(context);

        Self::new(context, vert, frag)
    }

    pub fn enable(&self, context: &GL) {
        context.use_program(Some(&self.compiled));
    }

    pub fn attribute_location(&self, name: &str) -> AttributeLocation {
        self.attribute_map.get(name).unwrap().to_owned()
    }

    pub fn uniform_location(&self, name: &str) -> UniformLocation {
        self.uniform_map.get(name).unwrap().to_owned()
    }
}

fn attribute_map(program: &WebGlProgram, context: &GL, vert: &VertexShader) -> AttributeMap {
    vert.attributes.iter().map(|name| {
        (name.to_string(), context.get_attrib_location(&program, name) as AttributeLocation)
    }).collect()
}

fn uniform_map(program: &WebGlProgram, context: &GL, vert: &VertexShader) -> UniformMap {
    vert.uniforms.iter().map(|name| {
        (name.to_string(), context.get_uniform_location(&program, name).unwrap())
    }).collect()
}
