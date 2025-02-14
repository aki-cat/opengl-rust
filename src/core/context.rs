use super::{Texture, Textures};

use super::enums::*;
use std::{os::raw::c_void, ptr::null};

pub struct Context {
    _private: *const (),
}

impl Context {
    const fn check_type_is_indeice(ty: super::GlType) -> bool {
        match ty {
            super::GlType::u8 => true,
            super::GlType::u16 => true,
            super::GlType::u32 => true,
            _ => false,
        }
    }

    /// Create a new context.
    pub fn new<F: FnMut(&'static str) -> *const c_void>(f: F) -> Self {
        gl::load_with(f);
        Self { _private: null() }
    }

    /// Create a new array object..
    ///
    /// # Note
    /// Call this methed is unsafe.
    ///
    /// If you do wish to use this method, you will need to manually
    /// manage the lifecycle of the memory. Improper handling can
    /// easily cause null pointer exceptions.
    pub unsafe fn new_array(&self) -> super::Array {
        super::Array::new()
    }

    /// Create multiple new array objeects.
    ///
    /// # Note
    /// Call this methed is unsafe.
    ///
    /// If you do wish to use this method, you will need to manually
    /// manage the lifecycle of the memory. Improper handling can
    /// easily cause null pointer exceptions.
    pub unsafe fn new_arrays(&self, count: usize) -> super::Arrays {
        super::Arrays::new(count)
    }

    /// Create a new buffer object.
    ///
    /// # Note
    /// Call this methed is unsafe.
    ///
    /// If you do wish to use this method, you will need to manually
    /// manage the lifecycle of the memory. Improper handling can
    /// easily cause null pointer exceptions.
    pub unsafe fn new_buffer(&self) -> super::Buffer {
        super::Buffer::new()
    }

    /// Create multiple new buffer objeects.
    ///
    /// # Note
    /// Call this methed is unsafe.
    ///
    /// If you do wish to use this method, you will need to manually
    /// manage the lifecycle of the memory. Improper handling can
    /// easily cause null pointer exceptions.
    pub unsafe fn new_buffers(&self, count: usize) -> super::Buffers {
        super::Buffers::new(count)
    }

    /// Create a new shader object.
    pub fn new_shader(&self, ty: super::ShaderType) -> super::Shader {
        super::Shader::new(ty)
    }

    /// Create a new program object.
    pub fn new_program(&self) -> super::Program {
        super::Program::new()
    }

    /// Create a new texture object.
    pub fn new_texture(&self) -> Texture {
        Texture::new()
    }

    /// Create multiple new texture objeects.
    pub fn new_textures(&self, count: usize) -> Textures {
        Textures::new(count)
    }

    /// Wrapper of `glViewport(...)`.
    pub fn view_port(&self, x: i32, y: i32, width: i32, height: i32) {
        unsafe {
            gl::Viewport(x, y, width, height);
        }
    }

    /// Wrapper of `glClearColor(...)`
    pub fn clear_color(&self, red: f32, green: f32, blue: f32, alpha: f32) {
        unsafe {
            gl::ClearColor(red, green, blue, alpha);
        }
    }

    /// Wrapper of `glClear(...)`
    pub fn clear(&self, mask: u32) {
        unsafe {
            gl::Clear(mask);
        }
    }

    /// Wrapper of `glEnable(...)`
    pub fn enable(&self, cap: Cap) {
        unsafe {
            gl::Enable(cap.to_gl_cap());
        }
    }

    /// Wrapper of `glDisable(...)`
    pub fn disable(&self, cap: Cap) {
        unsafe {
            gl::Disable(cap.to_gl_cap());
        }
    }

    /// Wrapper of `glDrawArrays(...)`
    pub fn draw_arrays(&self, mode: Mode, first: i32, count: i32) {
        unsafe {
            gl::DrawArrays(mode.to_gl_mode(), first, count);
        }
    }

    /// Warpper of `glDrawElements(...)`
    pub fn draw_elements(&self, mode: Mode, count: usize, ty: super::GlType, offset: usize) {
        if !Self::check_type_is_indeice(ty) {
            panic!("The type is not a index type.");
        }
        unsafe {
            gl::DrawElements(mode.to_gl_mode(), count as _, ty.to_gl_type(), offset as _);
        }
    }

    /// Unsafe version of `draw_elements(...)`
    ///
    /// # Note
    /// If `self.draw_elements(...)` is never panic,
    /// you can use this method to improve performance.
    pub unsafe fn draw_elements_unchecked(
        &self,
        mode: Mode,
        count: usize,
        ty: super::GlType,
        offset: usize,
    ) {
        gl::DrawElements(mode.to_gl_mode(), count as _, ty.to_gl_type(), offset as _);
    }
}
