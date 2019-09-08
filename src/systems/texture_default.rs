use specs::prelude::*;
use crate::resources::*;
use crate::components::*;
use crate::utilities::*;

pub struct TextureDefault;

impl<'a> System<'a> for TextureDefault {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, WebGlContext>,
        ReadStorage<'a, Geometry>,
        WriteStorage<'a, Texture>,
        WriteStorage<'a, BufferData>,
        WriteStorage<'a, Dimensions>,
        WriteStorage<'a, WebGlTexture>,
    );

    fn run(&mut self, (entities, context, geometries, mut textures, mut buffer_datas, mut dimensions, mut webgl_textures): Self::SystemData) {
        let without_textures = (&entities, &geometries, !&textures)
            .join().map(|(e, g, _)| (e, g)).collect::<Vec<_>>();

        let mut textures_to_add = vec![];
        let mut dimensions_to_add = vec![];

        for (entity, geometry) in without_textures {
            match textures.get(geometry.model) {
                None => {
                    let geometry_data = buffer_datas.get(geometry.model).unwrap();

                    let texcoords = create_blank_texcoords(geometry_data.iter().count());
                    let texture = create_blank_texture(&context, &[255, 255, 255, 255]);

                    let texture_model = entities.create();

                    buffer_datas.insert(texture_model, BufferData(texcoords)).unwrap();
                    webgl_textures.insert(texture_model, WebGlTexture(texture)).unwrap();
                    dimensions_to_add.push((texture_model, Dimensions(2)));

                    textures_to_add.push((geometry.model, Texture { model: texture_model }));
                    textures_to_add.push((entity, Texture { model: texture_model }));
                },
                Some(t) => textures_to_add.push((entity, t.clone())),
            };
        }

        for (entity, c) in textures_to_add {
            textures.insert(entity, c).unwrap();
        }

        for (entity, d) in dimensions_to_add {
            dimensions.insert(entity, d).unwrap();
        }
    }
}
