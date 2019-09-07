use specs::prelude::*;
use web_sys::HtmlImageElement;
use std::cell::RefCell;
use crate::components::*;
use crate::utilities::*;

thread_local! {
    static LOADED: RefCell<Vec<Entity>> = RefCell::new(vec![]);
}

pub struct ImageLoader;

impl<'a> System<'a> for ImageLoader {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, ImageToLoad>,
        WriteStorage<'a, Image>,
    );

    fn run(&mut self, (entities, mut images_to_load, mut images): Self::SystemData) {
        for (entity, image_to_load) in (&entities, &mut images_to_load).join() {
            if let Some(_) = image_to_load.image {
                continue;
            }

            let image = HtmlImageElement::new().unwrap();

            single_use_handler(move |_| {
                LOADED.with(|v| v.borrow_mut().push(entity));
            }, |h| {
                image.add_event_listener_with_callback("load", h).unwrap();
            });

            image.set_src(&image_to_load.src);

            image_to_load.image = Some(image);
        }

        LOADED.with(|v| {
            for entity in v.borrow().iter() {
                let image_to_load = images_to_load.remove(*entity).unwrap();

                images.insert(*entity, image_to_load.into()).unwrap();
            }

            v.borrow_mut().clear();
        });
    }
}
