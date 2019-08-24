use specs::prelude::*;
use wasm_bindgen::JsCast;

use super::resources::Window;
use super::resources::WebGlContext;

use web_sys::{Document, Element, Window as Win};
use web_sys::HtmlCanvasElement as Canvas;
use web_sys::WebGlRenderingContext as Context;

pub struct Webpage;

impl<'a> System<'a> for Webpage {
    type SystemData = ();

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let canvas = create_canvas(&document);
        let context = get_context(&canvas);
        let style = create_style(&document);

        add_to_page(&document, &canvas);
        add_to_page(&document, &style);
        resize_canvas(&window, &document, &canvas);

        let resource = Window::new(window, canvas);
        world.insert(resource);

        world.insert(WebGlContext(context));
    }

    fn run(&mut self, (): Self::SystemData) {

    }
}

fn create_canvas(document: &Document) -> Canvas {
    document.create_element("canvas").unwrap().dyn_into::<Canvas>().unwrap()
}

fn get_context(canvas: &Canvas) -> Context {
    canvas.get_context("webgl").unwrap().unwrap().dyn_into::<Context>().unwrap()
}

fn create_style(document: &Document) -> Element {
    let style = document.create_element("style").unwrap();
    let css = document.create_text_node("
        html, body, canvas {
            margin: 0;
            padding: 0;
            width: 100%;
            height: 100%;
            overflow: hidden;
        };
    ");

    style.append_child(&css).unwrap();
    style
}

fn add_to_page(document: &Document, element: &Element) {
    document.body().unwrap().append_child(element).unwrap();
}

fn resize_canvas(window: &Win, document: &Document, canvas: &Canvas) {
    let pixel_ratio = window.device_pixel_ratio();

    let body = document.body().unwrap();
    let rectangle = body.get_bounding_client_rect();

    let width = rectangle.width() * pixel_ratio;
    let height = rectangle.height() * pixel_ratio;

    canvas.set_width(width as u32);
    canvas.set_height(height as u32);
}
