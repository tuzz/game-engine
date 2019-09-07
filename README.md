## Game Engine

A project for me to explore writing a game engine from scratch using an
[ECS](https://en.wikipedia.org/wiki/Entity_component_system) design pattern. My
game engine is written in Rust, compiles to WebAssembly and binds to a WebGL
canvas. That means [it runs in a browser](http://tuzz.github.io/game-engine).

Currently, it supports the following features:

- 3D meshes built from vertex normals
- Surface normals calculated for models
- Phong lighting model (ambient, diffuse, specular)
- Multiple directional and point lights
- Orthographic and perspective cameras
- Multiple cameras, rendered to portions of the view
- Dynamic generation of GLSL shaders
- Materials, or colors specified per vertex
- Scene graph that's optimised for lazy updates
- Game loop that's frame rate independent
- Keyboard input event handling

## Usage

```sh
$ make setup
$ make build
$ make server
```

If you're not on a Mac, you'll need to install `binaryen` manually. There are
a few more Makefile commands you might find useful (e.g. `build-watch`). For
more examples, check out the
[wasm-bindgen documentation](https://rustwasm.github.io/docs/wasm-bindgen/).
