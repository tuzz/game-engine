use specs::prelude::*;
use tobj::{load_obj_buf, load_mtl_buf, Model, Material};
use std::collections::HashMap;
use std::io::BufReader;
use crate::components::*;

pub struct ModelLoader;

#[derive(SystemData)]
pub struct SysData<'a> {
    entities: Entities<'a>,
    models_to_load: WriteStorage<'a, ModelsToLoad>,
    file_contents: WriteStorage<'a, FileContent>,
    buffer_datas: WriteStorage<'a, BufferData>,
    dimensions: WriteStorage<'a, Dimensions>,
    names: WriteStorage<'a, Name>,
}

impl<'a> System<'a> for ModelLoader {
    type SystemData = SysData<'a>;

    fn run(&mut self, mut s: SysData) {
        let preloaded = (&s.entities, &s.models_to_load).join()
            .filter(|(_, m)| m.preloaded).map(|(e, _)| e).collect::<Vec<_>>();

        for entity in preloaded {
            let mut models_to_load = s.models_to_load.remove(entity).unwrap();
            let files_and_content = files_and_content(&mut s, &mut models_to_load);
            let (models, materials) = models_and_materials(files_and_content);

            log(&format!("{:?}", models));
            log(&format!("{:?}", materials));

            for (model, material_index) in models {
                let _material = &materials[material_index.unwrap()];

                create_entity(&mut s, &model, &model.mesh.positions, 3, "geometry");
                create_entity(&mut s, &model, &model.mesh.normals, 3, "normals");

                match create_entity(&mut s, &model, &model.mesh.texcoords, 2, "material") {
                    Some(_entity) => {
                        // add material component to texture coordinate entity
                    },
                    None => {
                        // create a new entity with the component
                    },
                }
            }
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum FileType { Object, Material }
type FilesAndContent = Vec<(FileType, String, String)>;

fn files_and_content(s: &mut SysData, models_to_load: &mut ModelsToLoad) -> FilesAndContent {
    let model_filenames = &[
        (FileType::Material, &models_to_load.material_filenames),
        (FileType::Object, &models_to_load.object_filenames),
    ];

    model_filenames.iter().flat_map(|(file_type, filenames)| {
        filenames.iter().map(|(filename, file_loader)| {
            let entity = file_loader.unwrap();

            let content = s.file_contents.remove(entity).unwrap();
            s.entities.delete(entity).unwrap();

            (*file_type, filename.clone(), content.0)
        }).collect::<Vec<_>>()
    }).collect()
}

fn models_and_materials(files_and_content: FilesAndContent) -> (Vec<(Model, Option<usize>)>, Vec<Material>) {
    let (mut models, mut materials) = (vec![], vec![]);
    let mut material_map = HashMap::new();

    for (file_type, filename, content) in &files_and_content {
        match file_type {
            FileType::Material => { material_map.insert(filename, content); },
            FileType::Object => {
                let mut reader = BufReader::new(content.as_bytes());

                let (m1, mut m2) = load_obj_buf(&mut reader, |p| {
                    let name = p.to_str().unwrap().to_string();
                    let material = material_map.get(&name).unwrap();

                    load_mtl_buf(&mut BufReader::new(material.as_bytes()))
                }).unwrap();

                for model in m1 {
                    let offset = materials.len();
                    let index = model.mesh.material_id.map(|i| i + offset);

                    models.push((model, index));
                }

                materials.append(&mut m2);
            },
        }
    }

    (models, materials)
}

fn create_entity(s: &mut SysData, model: &Model, field: &[f32], dimensions: u32, name_prefix: &str) -> Option<Entity> {
    if field.len() == 0 {
        return None;
    }

    let mesh = &model.mesh;
    let entity = s.entities.create();

    let data = mesh.indices.iter().map(|i| field[*i as usize]).collect::<Vec<_>>();
    let name = format!("{}_{}", name_prefix, model.name);

    s.buffer_datas.insert(entity, BufferData(data)).unwrap();
    s.dimensions.insert(entity, Dimensions(dimensions)).unwrap();
    s.names.insert(entity, Name(name)).unwrap();

    Some(entity)
}

use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
