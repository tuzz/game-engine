use specs::prelude::*;
use crate::components::*;

pub struct ModelPreloader;

#[derive(SystemData)]
pub struct SysData<'a> {
    entities: Entities<'a>,
    models_to_load: WriteStorage<'a, ModelsToLoad>,
    files_to_load: WriteStorage<'a, FileToLoad>,
}

impl<'a> System<'a> for ModelPreloader {
    type SystemData = SysData<'a>;

    fn run(&mut self, mut s: SysData) {
        start_loading_files(&mut s);
        check_if_files_are_loaded(&mut s);
    }
}

fn start_loading_files(s: &mut SysData) {
    for models_to_load in (&mut s.models_to_load).join() {
        if models_to_load.preloading || models_to_load.preloaded {
            continue;
        }

        let model_filenames = &mut [
            &mut models_to_load.material_filenames,
            &mut models_to_load.object_filenames,
        ];

        for filenames in model_filenames {
            for (filename, file_loader) in filenames.iter_mut() {
                let file_to_load = FileToLoad::new(filename);
                let entity = s.entities.create();

                s.files_to_load.insert(entity, file_to_load).unwrap();
                file_loader.replace(entity);
            }
        }

        models_to_load.preloading = true;
    }
}

fn check_if_files_are_loaded(s: &mut SysData) {
    for models_to_load in (&mut s.models_to_load).join() {
        if models_to_load.preloaded {
            continue;
        }

        models_to_load.preloading = false;

        let model_filenames = &mut [
            &mut models_to_load.material_filenames,
            &mut models_to_load.object_filenames,
        ];

        for filenames in model_filenames {
            for (_, file_loader) in filenames.iter_mut() {
                if s.files_to_load.get(file_loader.unwrap()).is_some() {
                    models_to_load.preloading = true;
                }
            }
        }

        if !models_to_load.preloading {
            models_to_load.preloaded = true;
        }
    }
}
