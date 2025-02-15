use std::ops::{Index, IndexMut};

use gl::types::GLuint;

use super::{
    FrameBufferAttachment, FrameBufferTarget, ImageFormat, ImageTarget, InternalFormat,
    PixelDataType, TexParam, TexParamPair, TexTarget, TextureTarget,
};

/// Texture object
pub struct Texture {
    id: GLuint,
}

impl Texture {
    #[inline]
    pub(crate) fn new() -> Self {
        let mut id = 0;
        unsafe { gl::GenTextures(1, &mut id) };
        Self { id }
    }
}

impl Drop for Texture {
    #[inline]
    fn drop(&mut self) {
        unsafe { gl::DeleteTextures(1, &self.id) };
    }
}

pub struct Textures {
    textures: Vec<Texture>,
}

/// Texture objects
impl Textures {
    #[inline]
    pub(super) fn new(count: usize) -> Self {
        let mut id = Vec::with_capacity(count);
        unsafe {
            gl::GenTextures(count as _, id.as_mut_ptr());
        }
        Self {
            textures: id.into_iter().map(|id| Texture { id }).collect(),
        }
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &Texture> {
        self.textures.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Texture> {
        self.textures.iter_mut()
    }

    #[inline]
    pub fn count(&self) -> usize {
        self.textures.len()
    }
}

impl IntoIterator for Textures {
    type Item = Texture;
    type IntoIter = std::vec::IntoIter<Texture>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.textures.into_iter()
    }
}

impl Index<usize> for Textures {
    type Output = Texture;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.textures[index]
    }
}

impl IndexMut<usize> for Textures {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.textures[index]
    }
}

impl Texture {
    /// Wrapper of `glBindTexture(...)`
    #[inline]
    pub fn bind(&self, target: TexTarget) {
        unsafe { gl::BindTexture(target.to_gl_target(), self.id) }
    }

    /// Wrapper of `glBindTexture(...)`
    #[inline]
    pub fn unbind(target: TexTarget) {
        unsafe { gl::BindTexture(target.to_gl_target(), 0) }
    }

    /// Wrapper of `glActiveTexture(...)`
    #[inline]
    pub fn active(index: u32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + index);
        }
    }

    /// Wrapper of `glTextureParameter{i|f|iv|fv}(...)`
    #[inline]
    pub fn set(&self, param: TexParam) {
        unsafe {
            match param.to_pair() {
                TexParamPair::GLf(pname, param) => {
                    gl::TextureParameterf(self.id, pname, param);
                }
                TexParamPair::GLi(pname, param) => {
                    gl::TextureParameteri(self.id, pname, param);
                }
                TexParamPair::GLiv(pname, param) => {
                    gl::TextureParameteriv(self.id, pname, param.as_ptr());
                }
            }
        }
    }

    /// Wrapper of `glGenerateTextureMipmap(...)`
    #[inline]
    pub fn gen_minmap(&self) {
        unsafe {
            gl::GenerateTextureMipmap(self.id);
        }
    }

    /// Wrapper of `glTexImage2D(...)`
    #[inline]
    pub fn load<T>(
        target: ImageTarget,
        internal_format: InternalFormat,
        (width, height): (u32, u32),
        format: ImageFormat,
        type_: PixelDataType,
        data: &[T],
    ) -> Result<(), String> {
        let err = unsafe {
            gl::TexImage2D(
                target.to_gl_target(),
                0,
                internal_format.to_gl_format() as _,
                width as _,
                height as _,
                0,
                format.to_gl_format(),
                type_.to_gl_type(),
                data.as_ptr() as _,
            );
            gl::GetError()
        };
        let err_enum = match err {
            gl::NO_ERROR => return Ok(()),
            gl::INVALID_ENUM => "GL_INVALID_ENUM",
            gl::INVALID_VALUE => "GL_INVALID_VALUE",
            gl::INVALID_OPERATION => "GL_INVALID_OPERATION",
            _ => "Unknown Error",
        };
        Err(format!("Load Texture Error: {}({})", err, err_enum))
    }

    /// Wrapper of `glTexImage2D(...)` without error check
    #[inline]
    pub unsafe fn load_unchecked<T>(
        target: ImageTarget,
        internal_format: InternalFormat,
        (width, height): (u32, u32),
        format: ImageFormat,
        type_: PixelDataType,
        data: &[T],
    ) {
        gl::TexImage2D(
            target.to_gl_target(),
            0,
            internal_format.to_gl_format() as _,
            width as _,
            height as _,
            0,
            format.to_gl_format(),
            type_.to_gl_type(),
            data.as_ptr() as _,
        );
    }

    /// Wrapper of `glFramebufferTexture2D(...)`
    #[inline]
    pub fn attach(
        &self,
        target: FrameBufferTarget,
        attach: FrameBufferAttachment,
        tex_target: TextureTarget,
    ) {
        unsafe {
            gl::FramebufferTexture2D(
                target.to_gl_target(),
                attach.to_gl_attachment(),
                tex_target.to_gl_enum(),
                self.id,
                0,
            );
        }
    }
}
