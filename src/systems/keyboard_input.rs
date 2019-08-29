use specs::prelude::*;
use web_sys::KeyboardEvent;
use std::cell::RefCell;
use crate::resources::*;
use crate::utilities::*;

thread_local! {
    static KEY_EVENTS: RefCell<Vec<(Key, bool)>> = RefCell::new(vec![]);
}

const DOWN: bool = false;
const UP: bool = true;

pub struct KeyboardInput;

impl<'a> System<'a> for KeyboardInput {
    type SystemData = Write<'a, Keyboard>;

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

    fn run(&mut self, mut keyboard: Self::SystemData) {
        keyboard.just_pressed.clear();
        keyboard.just_released.clear();

        KEY_EVENTS.with(|v| {
            for (key, event) in v.borrow().iter() {
                match *event {
                    DOWN => {
                        keyboard.pressing.add(*key as u32);
                        keyboard.just_pressed.add(*key as u32);
                    },
                    UP => {
                        keyboard.pressing.remove(*key as u32);
                        keyboard.just_released.add(*key as u32);
                    },
                }
            }

            v.borrow_mut().clear();
        });
    }
}

fn handle_keydown<E: Into<KeyboardEvent>>(event: E) {
    let event = event.into();

    if event.repeat() {
        return;
    }

    let code = event.key_code();

    if let Some(key) = Key::lookup(code) {
        KEY_EVENTS.with(|v| v.borrow_mut().push((key, DOWN)))
    }
}

fn handle_keyup<E: Into<KeyboardEvent>>(event: E) {
    let code = event.into().key_code();

    if let Some(key) = Key::lookup(code) {
        KEY_EVENTS.with(|v| v.borrow_mut().push((key, UP)))
    }
}
