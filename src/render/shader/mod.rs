mod vertex;
mod fragment;

pub use vertex::VertexShader;
pub use fragment::FragmentShader;

use web_sys::WebGlRenderingContext as GL;
use web_sys::WebGlShader;

pub trait Shader {
    fn default(context: &GL) -> Self;

    fn compile(context: &GL, kind: u32, source: &str) -> WebGlShader {
        let shader = context.create_shader(kind).unwrap();

        context.shader_source(&shader, source);
        context.compile_shader(&shader);

        shader
    }
}
