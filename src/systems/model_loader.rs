use specs::prelude::*;
use tobj::{load_obj_buf, load_mtl_buf};
use std::collections::HashMap;
use std::io::BufReader;
use crate::resources::*;
use crate::components::*;

pub struct ModelLoader;

#[derive(SystemData)]
pub struct SysData<'a> {
    entities: Entities<'a>,
    model_groups: Write<'a, ModelGroups>,

    models_to_load: WriteStorage<'a, ModelsToLoad>,
    file_contents: WriteStorage<'a, FileContent>,
    buffer_datas: WriteStorage<'a, BufferData>,
    dimensions: WriteStorage<'a, Dimensions>,
    names: WriteStorage<'a, Name>,

    ambients: WriteStorage<'a, Ambient>,
    diffuses: WriteStorage<'a, Diffuse>,
    speculars: WriteStorage<'a, Specular>,
    shinies: WriteStorage<'a, Shininess>,

    images_to_load: WriteStorage<'a, ImageToLoad>,

    normals: WriteStorage<'a, Normals>,
    materials: WriteStorage<'a, Material>,
    textures: WriteStorage<'a, Texture>,
    texcoords: WriteStorage<'a, TexCoords>,
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

            let materials_and_textures = materials.iter().map(|material| {
                let material_model = create_material_entity(&mut s, &material);
                let texture_model = create_texture_entity(&mut s, &material);

                (material_model, texture_model)
            }).collect::<Vec<_>>();

            for (model, material_index, filename) in models {
                let material_and_texture = material_index.map(|i| materials_and_textures[i]);

                let geometry_model = create_buffer_entity(&mut s, &model, &model.mesh.positions, 3, "geometry").unwrap();
                let normals_model = create_buffer_entity(&mut s, &model, &model.mesh.normals, 3, "normals");
                let texcoords_model = create_buffer_entity(&mut s, &model, &model.mesh.texcoords, 2, "texcoords");

                if let Some(model) = normals_model {
                    s.normals.insert(geometry_model, Normals { model }).unwrap();
                }

                if let Some(model) = texcoords_model {
                    s.texcoords.insert(geometry_model, TexCoords { model }).unwrap();
                }

                if let Some((model, _)) = material_and_texture {
                    s.materials.insert(geometry_model, Material { model }).unwrap();
                }

                if let Some((_, Some(model))) = material_and_texture {
                    s.textures.insert(geometry_model, Texture { model }).unwrap();
                }

                s.model_groups.add(filename, &geometry_model);
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

fn models_and_materials(files_and_content: FilesAndContent) -> (Vec<(tobj::Model, Option<usize>, String)>, Vec<tobj::Material>) {
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

                    models.push((model, index, filename.clone()));
                }

                materials.append(&mut m2);
            },
        }
    }

    (models, materials)
}

fn create_material_entity(s: &mut SysData, material: &tobj::Material) -> Entity {
    let entity = s.entities.create();

    let ambient = material.ambient.into();
    let diffuse = material.diffuse.into();
    let specular = material.specular.into();
    let shininess = material.shininess.into();

    let name = format!("material_{}", material.name);

    s.ambients.insert(entity, Ambient(ambient)).unwrap();
    s.diffuses.insert(entity, Diffuse(diffuse)).unwrap();
    s.speculars.insert(entity, Specular(specular)).unwrap();
    s.shinies.insert(entity, Shininess(shininess)).unwrap();
    s.names.insert(entity, Name(name)).unwrap();

    entity
}

fn create_texture_entity(s: &mut SysData, material: &tobj::Material) -> Option<Entity> {
    let entity = s.entities.create();

    let filenames = [
        &material.ambient_texture,
        &material.diffuse_texture,
        &material.specular_texture,
        &material.normal_texture,
        &material.dissolve_texture,
    ];

    let filename = filenames.iter().find(|f| !f.is_empty()).cloned()?;

    s.images_to_load.insert(entity, ImageToLoad::new(filename)).unwrap();
    s.names.insert(entity, Name(filename.to_string())).unwrap();

    Some(entity)
}

fn create_buffer_entity(s: &mut SysData, model: &tobj::Model, field: &[f32], dimensions: u32, name_prefix: &str) -> Option<Entity> {
    if field.len() == 0 {
        return None;
    }

    let mesh = &model.mesh;
    let entity = s.entities.create();
    let d = dimensions as usize;

    let data = mesh.indices.iter()
        .flat_map(|&i| field[d * i as usize..].iter().take(d).cloned())
        .collect::<Vec<_>>();

    let name = format!("{}_{}", name_prefix, model.name);

    s.buffer_datas.insert(entity, BufferData(data)).unwrap();
    s.dimensions.insert(entity, Dimensions(dimensions)).unwrap();
    s.names.insert(entity, Name(name)).unwrap();

    Some(entity)
}
