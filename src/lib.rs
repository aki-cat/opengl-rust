mod core;
mod ext;

pub use core::*;
pub use ext::*;

pub const OPENGL_VERSION: (u32, u32) = (4, 5);

pub extern crate mats;
