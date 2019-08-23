use super::Shader;

use web_sys::WebGlRenderingContext as GL;
use web_sys::WebGlShader;

type V = Vec<&'static str>;

pub struct VertexShader {
    pub compiled: WebGlShader,
    pub attributes: V,
    pub uniforms: V,
}

impl VertexShader {
    fn new(context: &GL, source: &str, attributes: V, uniforms: V) -> Self {
        let compiled = Self::compile(context, GL::VERTEX_SHADER, source);

        Self { compiled, attributes, uniforms }
    }
}

impl Shader for VertexShader {
    fn default(context: &GL) -> Self {
        let source = "
            attribute vec2 a_position;

            attribute vec4 a_color;
            varying vec4 v_color;

            uniform mat3 u_matrix;

            void main() {
              gl_Position = vec4((u_matrix * vec3(a_position, 1)).xy, 0, 1);
              v_color = a_color;
            }
        ";

        Self::new(context, source, vec!["a_position", "a_color"], vec!["u_matrix"])
    }
}
