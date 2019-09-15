#[macro_use]
extern crate specs_derive;

#[macro_use]
extern crate shred_derive;

#[macro_use]
extern crate impl_ops;

#[macro_use] #[cfg(test)]
extern crate assert_approx_eq;

#[macro_use]
mod utilities;
mod components;
mod resources;
mod systems;

use specs::prelude::*;
use wasm_bindgen::prelude::*;
use utilities::GameLoop;
use systems::*;

#[wasm_bindgen(start)]
pub fn main() {
    let mut game_loop = GameLoop::new();

    let mut webpage = Webpage;
    let mut panic_to_console = PanicToConsole;
    let mut scene_loader = SceneLoader::default();
    let mut model_preloader = ModelPreloader;
    let mut file_loader = FileLoader;
    let mut image_loader = ImageLoader;
    let mut model_loader = ModelLoader;
    let mut name_indexer = NameIndexer::default();
    let mut group_expander = GroupExpander;
    let mut vertex_normals = VertexNormals;
    let mut shader_compiler = ShaderCompiler;
    let mut location_lookup = LocationLookup;
    let mut material_default = MaterialDefault;
    let mut coloring_default = ColoringDefault;
    let mut texture_default = TextureDefault;
    let mut texcoords_default = TexcoordsDefault;
    let mut webgl_texture = WebGlTexture::default();
    let mut webgl_buffer = WebGlBuffer;
    let mut use_program = UseProgram;
    let mut webgl_render = WebGlRender;
    let mut hierarchy = Hierarchy::new(&mut game_loop.world);
    let mut scene_graph = SceneGraph::default();
    let mut inverse_world = InverseWorld::default();
    let mut animation = Animation;
    let mut keyboard_input = KeyboardInput;

    game_loop.before(|world| {
        System::setup(&mut webpage, world);
        System::setup(&mut panic_to_console, world);
        System::setup(&mut scene_loader, world);
        System::setup(&mut model_preloader, world);
        System::setup(&mut file_loader, world);
        System::setup(&mut image_loader, world);
        System::setup(&mut model_loader, world);
        System::setup(&mut name_indexer, world);
        System::setup(&mut group_expander, world);
        System::setup(&mut vertex_normals, world);
        System::setup(&mut shader_compiler, world);
        System::setup(&mut location_lookup, world);
        System::setup(&mut material_default, world);
        System::setup(&mut coloring_default, world);
        System::setup(&mut texture_default, world);
        System::setup(&mut texcoords_default, world);
        System::setup(&mut webgl_texture, world);
        System::setup(&mut webgl_buffer, world);
        System::setup(&mut use_program, world);
        System::setup(&mut webgl_render, world);
        System::setup(&mut scene_graph, world);
        System::setup(&mut inverse_world, world);
        System::setup(&mut animation, world);
        System::setup(&mut keyboard_input, world);
    });

    game_loop.run(move |world| {
        keyboard_input.run_now(world);
    }, move |world| {
        scene_loader.run_now(world);
        model_preloader.run_now(world);
        file_loader.run_now(world);
        image_loader.run_now(world);
        model_loader.run_now(world);
        name_indexer.run_now(world);
        group_expander.run_now(world);
        vertex_normals.run_now(world);
        material_default.run_now(world);
        coloring_default.run_now(world);
        texture_default.run_now(world);
        texcoords_default.run_now(world);
        webgl_texture.run_now(world);
        webgl_buffer.run_now(world);
        animation.run_now(world);
        hierarchy.run_now(world);
        scene_graph.run_now(world);
        inverse_world.run_now(world);
        use_program.run_now(world);
        webgl_render.run_now(world);
    });
}
