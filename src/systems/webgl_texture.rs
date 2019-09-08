use specs::prelude::*;
use web_sys::WebGlRenderingContext as GL;
use crate::resources::*;
use crate::components::WebGlTexture as Texture;
use crate::components::*;

#[derive(Default)]
pub struct WebGlTexture {
    image_reader: Option<ReaderId<ComponentEvent>>,
}

#[derive(SystemData)]
pub struct SysData<'a> {
    entities: Entities<'a>,
    context: ReadExpect<'a, WebGlContext>,

    images_to_load: ReadStorage<'a, ImageToLoad>,
    images: ReadStorage<'a, Image>,

    textures: WriteStorage<'a, Texture>,
}

impl<'a> System<'a> for WebGlTexture {
    type SystemData = SysData<'a>;

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        self.image_reader = Some(
            WriteStorage::<Image>::fetch(world).register_reader()
        );
    }

    fn run(&mut self, mut s: SysData) {
        let mut textures_to_add = vec![];

        for entity in self.entities_whose_image_has_changed(&mut s) {
            let texture = create_or_update_texture_from_image(&mut s, entity);
            textures_to_add.push((entity, texture));
        }

        for (entity, _, _) in (&s.entities, &s.images_to_load, !&s.textures).join() {
            textures_to_add.push((entity, create_blank_texture(&s.context)));
        };

        for (entity, texture) in textures_to_add {
            s.textures.insert(entity, texture).unwrap();
        }
    }
}

impl WebGlTexture {
    fn entities_whose_image_has_changed(&mut self, s: &mut SysData) -> Vec<Entity> {
        let reader_id = self.image_reader.as_mut().unwrap();
        let mut changed = BitSet::new();

        for event in s.images.channel().read(reader_id) {
            match event {
                ComponentEvent::Inserted(id) => changed.add(*id),
                ComponentEvent::Modified(id) => changed.add(*id),
                ComponentEvent::Removed(_) => { panic!("Not supported") },
            };
        }

        (changed, &s.entities).join().map(|(_, e)| e).collect()
    }
}

fn create_or_update_texture_from_image(s: &mut SysData, entity: Entity) -> Texture {
    let texture = s.textures.remove(entity).unwrap_or_else(|| {
        Texture(s.context.create_texture().unwrap())
    });

    let image = s.images.get(entity).unwrap();

    s.context.bind_texture(GL::TEXTURE_2D, Some(&texture));
    fill_texture_from_image(&s.context, &image);

    texture
}

fn create_blank_texture(context: &GL) -> Texture {
    let texture = Texture(context.create_texture().unwrap());

    context.bind_texture(GL::TEXTURE_2D, Some(&texture));
    fill_texture_with_a_single_pixel(context, &[0, 0, 0, 255]);

    texture
}

fn fill_texture_from_image(context: &GL, image: &Image) {
    let target = GL::TEXTURE_2D;
    let level = 0;
    let internal_format = GL::RGBA as i32;
    let format = GL::RGBA;
    let type_ = GL::UNSIGNED_BYTE;

    context.tex_image_2d_with_u32_and_u32_and_image(
        target, level, internal_format, format, type_, image
    ).unwrap();
}

fn fill_texture_with_a_single_pixel(context: &GL, color: &[u8]) {
    let target = GL::TEXTURE_2D;
    let level = 0;
    let internal_format = GL::RGBA as i32;
    let width = 1;
    let height = 1;
    let border = 0;
    let format = GL::RGBA;
    let type_ = GL::UNSIGNED_BYTE;

    context.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
        target, level, internal_format, width, height, border, format, type_, Some(color),
    ).unwrap();
}
