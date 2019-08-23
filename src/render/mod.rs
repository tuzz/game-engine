use specs::prelude::*;
use wasm_bindgen::prelude::*;
use super::resources::Window;

use web_sys::WebGlRenderingContext as GL;
use web_sys::WebGlProgram as Program;
use web_sys::WebGlShader as Shader;
use web_sys::HtmlCanvasElement as Canvas;
use web_sys::WebGlRenderingContext as Context;

use js_sys::Float32Array;

pub struct Render;

impl<'a> System<'a> for Render {
    type SystemData = ();

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        let window = window_resource(world);
        let canvas = &window.canvas;
        let context = &window.context;

        let vertex_shader = create_shader(context, GL::VERTEX_SHADER, "
            attribute vec2 a_position;

            attribute vec4 a_color;
            varying vec4 v_color;

            uniform mat3 u_matrix;

            void main() {
              gl_Position = vec4((u_matrix * vec3(a_position, 1)).xy, 0, 1);
              v_color = a_color;
            }
        ");

        let fragment_shader = create_shader(context, GL::FRAGMENT_SHADER, "
            precision mediump float;

            varying vec4 v_color;

            void main() {
              gl_FragColor = v_color; // return redish-purple
            }
        ");

        let program = create_program(context, &vertex_shader, &fragment_shader);

        let a_position = context.get_attrib_location(&program, "a_position");
        let a_color = context.get_attrib_location(&program, "a_color");
        let u_matrix = context.get_uniform_location(&program, "u_matrix").unwrap();

        let position_buffer = context.create_buffer().unwrap();
        context.bind_buffer(GL::ARRAY_BUFFER, Some(&position_buffer));
        let positions = [
            0.0, 0.0,
            0.0, 0.5,
            0.7, 0.0,
        ];

        unsafe {
        context.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &Float32Array::view(&positions), GL::STATIC_DRAW);
        }

        //
        let color_buffer = context.create_buffer().unwrap();
        context.bind_buffer(GL::ARRAY_BUFFER, Some(&color_buffer));
        let colors = [
            1.0, 0.5, 0.5, 1.0,
            0.5, 1.0, 0.5, 1.0,
            0.5, 0.5, 1.0, 1.0,
        ];

        unsafe {
        context.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &Float32Array::view(&colors), GL::STATIC_DRAW);
        }
        //

        clear_viewport(&canvas, &context);

        context.use_program(Some(&program));

        context.enable_vertex_attrib_array(a_position as u32);
        context.bind_buffer(GL::ARRAY_BUFFER, Some(&position_buffer));
        context.vertex_attrib_pointer_with_i32(a_position as u32, 2, GL::FLOAT, false, 0, 0);

        //
        context.enable_vertex_attrib_array(a_color as u32);
        context.bind_buffer(GL::ARRAY_BUFFER, Some(&color_buffer));
        context.vertex_attrib_pointer_with_i32(a_color as u32, 4, GL::FLOAT, false, 0, 0);
        //

        let matrix = [
            1.0, 0.0, 0.0,
            0.0, 1.0, 0.0,
            0.0, 0.0, 1.0,
        ];
        context.uniform_matrix3fv_with_f32_array(Some(&u_matrix), false, &matrix);
        context.draw_arrays(GL::TRIANGLES, 0, 3);
    }

    fn run(&mut self, (): Self::SystemData) {
        log("Hello, console!");
    }
}

fn window_resource(world: &mut World) -> &mut Window {
    world.get_mut::<Window>().unwrap()
}

fn create_shader(context: &GL, kind: u32, source: &str) -> Shader {
    let shader = context.create_shader(kind).unwrap();

    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    shader
}

fn create_program(context: &GL, vert: &Shader, frag: &Shader) -> Program {
    let program = context.create_program().unwrap();

    context.attach_shader(&program, vert);
    context.attach_shader(&program, frag);

    context.link_program(&program);

    program
}

fn clear_viewport(canvas: &Canvas, context: &Context) {
    let width = canvas.width() as i32;
    let height = canvas.height() as i32;

    context.viewport(0, 0, width, height);
    context.clear_color(0.0, 0.0, 0.0, 0.0);
    context.clear(GL::COLOR_BUFFER_BIT);
}

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
