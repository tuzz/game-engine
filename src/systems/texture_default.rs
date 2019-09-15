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
        WriteStorage<'a, WebGlTexture>,
    );

    fn run(&mut self, (entities, context, geometries, mut textures, mut webgl_textures): Self::SystemData) {
        let without_textures = (&entities, &geometries, !&textures)
            .join().map(|(e, g, _)| (e, g)).collect::<Vec<_>>();

        let mut textures_to_add = vec![];

        for (entity, geometry) in without_textures {
            match textures.get(geometry.model) {
                None => {
                    let texture = create_blank_texture(&context, &[255, 255, 255, 255]);
                    let texture_model = entities.create();

                    webgl_textures.insert(texture_model, WebGlTexture(texture)).unwrap();

                    textures_to_add.push((geometry.model, Texture { model: texture_model }));
                    textures_to_add.push((entity, Texture { model: texture_model }));
                },
                Some(t) => textures_to_add.push((entity, t.clone())),
            };
        }

        for (entity, c) in textures_to_add {
            textures.insert(entity, c).unwrap();
        }
    }
}
