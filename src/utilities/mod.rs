mod game_loop;
mod vector3f;
mod matrix4f;
mod triangle;
mod shader;
mod blank_texture;
mod event_handlers;

#[macro_use]
mod debugging;

pub use std::f32::consts::PI;

pub use game_loop::GameLoop;
pub use vector3f::Vector3f;
pub use matrix4f::Matrix4f;
pub use triangle::Triangle;
pub use shader::Shader;
pub use blank_texture::*;
pub use event_handlers::*;
pub use debugging::*;

#[cfg(test)]
mod test_helpers;
