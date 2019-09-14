use specs::prelude::*;

use web_sys::Response;
use wasm_bindgen::JsValue;
use std::cell::RefCell;

use crate::resources::*;
use crate::components::*;
use crate::utilities::*;

thread_local! {
    static RESPONSES: RefCell<Vec<(Entity, JsValue)>> = RefCell::new(vec![]);
    static TEXTS:     RefCell<Vec<(Entity, JsValue)>> = RefCell::new(vec![]);
}

pub struct FileLoader;

impl<'a> System<'a> for FileLoader {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, BrowserWindow>,
        WriteStorage<'a, FileToLoad>,
        WriteStorage<'a, FileContent>,
    );

    fn run(&mut self, (entities, window, mut files_to_load, mut file_contents): Self::SystemData) {
        for (entity, file_to_load) in (&entities, &mut files_to_load).join() {
            if file_to_load.loading {
                continue;
            }

            register_closure_handler(move |response| {
                RESPONSES.with(|v| v.borrow_mut().push((entity, response)));
            }, |h| {
                window.fetch_with_str(&file_to_load.src).then(h);
            });

            file_to_load.loading = true;
        }

        RESPONSES.with(|ref_cell| {
            let responses = ref_cell.replace(vec![]);

            for (entity, response) in responses {
                let response: Response = response.into();

                register_closure_handler(move |text| {
                    TEXTS.with(|v| v.borrow_mut().push((entity, text)));
                }, |h| {
                    response.text().unwrap().then(h);
                });
            }
        });

        TEXTS.with(|ref_cell| {
            let texts = ref_cell.replace(vec![]);

            for (entity, text) in texts {
                let content = text.as_string().unwrap();

                files_to_load.remove(entity).unwrap();
                file_contents.insert(entity, FileContent(content)).unwrap();
            }
        });
    }
}
