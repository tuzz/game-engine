use specs::prelude::*;
use wasm_bindgen::prelude::*;

use super::components::Ticks;

#[derive(Default)]
pub struct Render {
    count: u32,
    p: Option<web_sys::Element>,
}

impl<'a> System<'a> for Render {
    type SystemData = ReadStorage<'a, Ticks>;

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();
        let p = document.create_element("p").unwrap();

        body.append_child(&p).unwrap();

        self.p = Some(p);
    }

    fn run(&mut self, ticks: Self::SystemData) {
        self.count += 1;
        let p = self.p.as_ref().unwrap();

        for ticks in ticks.join() {
            let s = format!("Render {}, Ticks: {}", self.count, ticks.count);
            p.set_inner_html(&s);
        }

        log("Hello, console!");
    }
}

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
