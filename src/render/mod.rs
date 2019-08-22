use specs::prelude::*;
use wasm_bindgen::prelude::*;

pub struct Render;

impl<'a> System<'a> for Render {
    type SystemData = ();

    fn run(&mut self, (): Self::SystemData) {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();
        let p = document.create_element("p").unwrap();

        p.set_inner_html("Hello, world!");
        body.append_child(&p).unwrap();

        log("Hello, console!");
    }
}

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
