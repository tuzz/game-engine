use specs::prelude::*;
use wasm_bindgen::prelude::*;

pub struct Render;

impl<'a> System<'a> for Render {
    type SystemData = ();

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
    }

    fn run(&mut self, (): Self::SystemData) {
        log("Hello, console!");
    }
}

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
