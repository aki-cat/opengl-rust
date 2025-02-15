use super::{Texture, Textures};

use super::enums::*;
use std::{os::raw::c_void, ptr::null};

pub struct Context {
    _private: *const (),
}

impl Context {
    #[inline]
    const fn check_type_is_indeice(ty: super::GlType) -> bool {
        match ty {
            super::GlType::u8 => true,
            super::GlType::u16 => true,
            super::GlType::u32 => true,
            _ => false,
        }
    }

    /// Create a new context.
    #[inline]
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
    #[inline]
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
    #[inline]
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
    #[inline]
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
    #[inline]
    pub unsafe fn new_buffers(&self, count: usize) -> super::Buffers {
        super::Buffers::new(count)
    }

    /// Create a new shader object.
    #[inline]
    pub fn new_shader(&self, ty: super::ShaderType) -> super::Shader {
        super::Shader::new(ty)
    }

    /// Create a new program object.
    #[inline]
    pub fn new_program(&self) -> super::Program {
        super::Program::new()
    }

    /// Create a new texture object.
    #[inline]
    pub fn new_texture(&self) -> Texture {
        Texture::new()
    }

    /// Create multiple new texture objeects.
    #[inline]
    pub fn new_textures(&self, count: usize) -> Textures {
        Textures::new(count)
    }

    /// Wrapper of `glViewport(...)`.
    #[inline]
    pub fn view_port(&self, x: i32, y: i32, width: i32, height: i32) {
        unsafe {
            gl::Viewport(x, y, width, height);
        }
    }

    /// Wrapper of `glClearColor(...)`
    #[inline]
    pub fn clear_color(&self, red: f32, green: f32, blue: f32, alpha: f32) {
        unsafe {
            gl::ClearColor(red, green, blue, alpha);
        }
    }

    /// Wrapper of `glClear(...)`
    #[inline]
    pub fn clear(&self, mask: u32) {
        unsafe {
            gl::Clear(mask);
        }
    }

    /// Wrapper of `glEnable(...)`
    #[inline]
    pub fn enable(&self, cap: Cap) {
        unsafe {
            gl::Enable(cap.to_gl_cap());
        }
    }

    /// Wrapper of `glDisable(...)`
    #[inline]
    pub fn disable(&self, cap: Cap) {
        unsafe {
            gl::Disable(cap.to_gl_cap());
        }
    }

    /// Wrapper of `glDepthMask(...)`
    #[inline]
    pub fn depth_mask(&self, mask: bool) {
        unsafe {
            gl::DepthMask(mask as _);
        }
    }

    /// Wrapper of `glDepthFunc(...)`
    #[inline]
    pub fn depth_func(&self, func: DepthFunc) {
        unsafe {
            gl::DepthFunc(func.to_gl_func());
        }
    }

    /// Wrapper of `glStencilMask(...)`
    #[inline]
    pub fn stencil_mask(&self, mask: u32) {
        unsafe {
            gl::StencilMask(mask);
        }
    }

    /// Wrapper of `glStencilFunc(...)`
    #[inline]
    pub fn stencil_func(&self, func: StencilFunc, ref_: i32, mask: u32) {
        unsafe {
            gl::StencilFunc(func.to_gl_func(), ref_, mask);
        }
    }

    /// Wrapper of `glStencilOp(...)`
    #[inline]
    pub fn stencil_op(&self, fail: StencilOp, zfail: StencilOp, zpass: StencilOp) {
        unsafe {
            gl::StencilOp(fail.to_gl_op(), zfail.to_gl_op(), zpass.to_gl_op());
        }
    }

    /// Wrapper of `glBlendFunc(...)`
    #[inline]
    pub fn blend_func(&self, src: BlendFactor, dst: BlendFactor) {
        unsafe {
            gl::BlendFunc(src.to_gl_func(), dst.to_gl_func());
        }
    }

    /// Wrapper of `glBlendFuncSeparate(...)`
    #[inline]
    pub fn blend_func_separate(
        &self,
        src: BlendFactor,
        dst: BlendFactor,
        src_alpha: BlendFactor,
        dst_alpha: BlendFactor,
    ) {
        unsafe {
            gl::BlendFuncSeparate(
                src.to_gl_func(),
                dst.to_gl_func(),
                src_alpha.to_gl_func(),
                dst_alpha.to_gl_func(),
            );
        }
    }

    #[inline]
    pub fn blend_equation(&self, mode: BlendEquation) {
        unsafe {
            gl::BlendEquation(mode.to_gl_equation());
        }
    }

    /// Wrapper of `glCullFace(...)`
    #[inline]
    pub fn cull_face(&self, face: CullFace) {
        unsafe {
            gl::CullFace(face.to_gl_face());
        }
    }

    /// Wrapper of `glFrontFace(...)`
    #[inline]
    pub fn front_face(&self, face: FrontFace) {
        unsafe {
            gl::FrontFace(face.to_gl_face());
        }
    }

    /// Wrapper of `glDrawArrays(...)`
    #[inline]
    pub fn draw_arrays(&self, mode: Mode, first: i32, count: i32) {
        unsafe {
            gl::DrawArrays(mode.to_gl_mode(), first, count);
        }
    }

    /// Warpper of `glDrawElements(...)`
    #[inline]
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
    #[inline]
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
