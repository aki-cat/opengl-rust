//! Here defines the Rust style encapsulation of the original OpenGL API.
//!
//! # Note
//! Due to the complex dependencies of OpenGL objects,
//! these contents are not strictly safe.

mod array;
mod buffer;
mod context;
mod enums;
mod frame_buffer;
mod program;
mod shader;
mod texture;

pub use array::{Array, Arrays};
pub use buffer::{Buffer, Buffers, GlType};
pub use context::Context;
pub use enums::*;
pub use frame_buffer::{FrameBuffer, FrameBuffers};
pub use program::Program;
pub use shader::Shader;
pub use texture::{Texture, Textures};

trait GlTypeT {
    fn to_gl_type() -> gl::types::GLenum;
}

macro_rules! impl_gl_type {
    ($($type:ty => $gl_type:expr);+ $(;)?) => {
        $(
            impl GlTypeT for $type {
                #[inline]
                fn to_gl_type() -> gl::types::GLenum {
                    $gl_type
                }
            }
        )+
    };
}

impl_gl_type! {
    u8 => gl::UNSIGNED_BYTE;
    i8 => gl::BYTE;
    u16 => gl::UNSIGNED_SHORT;
    i16 => gl::SHORT;
    u32 => gl::UNSIGNED_INT;
    i32 => gl::INT;
    f32 => gl::FLOAT;
    f64 => gl::DOUBLE;
}
