use specs::prelude::*;
use crate::resources::*;

pub struct PanicToConsole;

impl<'a> System<'a> for PanicToConsole {
    type SystemData = ();

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        let window = world.fetch::<BrowserWindow>();

        make_backtrace_less_noisy(&window);

        console_error_panic_hook::set_once();
    }

    fn run(&mut self, (): Self::SystemData) {
        unimplemented!()
    }
}

fn make_backtrace_less_noisy(window: &BrowserWindow) {
    let document = window.document().unwrap();
    let body = document.body().unwrap();
    let script = document.create_element("script").unwrap();

    let inner = document.create_text_node("
        try {
            Error.stackTraceLimit = 0;
        } catch(e) {
            // not supported
        }
    ");

    script.append_child(&inner).unwrap();
    body.append_child(&script).unwrap();
}
