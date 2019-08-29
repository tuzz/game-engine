use specs::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::KeyboardEvent;
use crate::resources::*;
use crate::utilities::*;

pub struct KeyboardInput;

impl<'a> System<'a> for KeyboardInput {
    type SystemData = ();

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        let window = world.fetch::<BrowserWindow>();

        register_handler(handle_keydown, |h| {
            window.add_event_listener_with_callback("keydown", h).unwrap();
        });

        register_handler(handle_keyup, |h| {
            window.add_event_listener_with_callback("keyup", h).unwrap();
        });
    }

    fn run(&mut self, (): Self::SystemData) {
        unimplemented!()
    }
}

fn handle_keydown<E: Into<KeyboardEvent>>(event: E) {
    let key = event.into().key();

    log(&format!("pressed {:?}", key));
}

fn handle_keyup<E: Into<KeyboardEvent>>(event: E) {
    let key = event.into().key();

    log(&format!("unpressed {:?}", key));
}

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
