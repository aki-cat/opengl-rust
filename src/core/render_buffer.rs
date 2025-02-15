use gl::types::GLuint;

use super::RenderBufferFormat;

pub struct RenderBuffer {
    pub(super) rbo: GLuint,
}

impl RenderBuffer {
    #[inline]
    pub(super) fn new() -> Self {
        let mut rbo = 0;
        unsafe {
            gl::GenRenderbuffers(1, &mut rbo);
        }
        Self { rbo }
    }
}

impl Drop for RenderBuffer {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            gl::DeleteRenderbuffers(1, &self.rbo);
        }
    }
}

pub struct RenderBuffers {
    rbos: Vec<RenderBuffer>,
}

impl RenderBuffers {
    #[inline]
    pub(super) fn new(count: usize) -> Self {
        let mut rbos = Vec::with_capacity(count);
        unsafe {
            gl::GenRenderbuffers(count as i32, rbos.as_mut_ptr());
        }
        Self {
            rbos: rbos.into_iter().map(|rbo| RenderBuffer { rbo }).collect(),
        }
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &RenderBuffer> {
        self.rbos.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut RenderBuffer> {
        self.rbos.iter_mut()
    }
}

pub struct IntoIter {
    rbos: Vec<Option<RenderBuffer>>,
    index: usize,
}

impl IntoIter {
    #[inline]
    fn new(rbos: Vec<RenderBuffer>) -> Self {
        Self {
            rbos: rbos.into_iter().map(|rbo| Some(rbo)).collect(),
            index: 0,
        }
    }
}

impl Iterator for IntoIter {
    type Item = RenderBuffer;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let result = self.rbos.get_mut(self.index)?.take();
        self.index += 1;
        result
    }
}

impl IntoIterator for RenderBuffers {
    type Item = RenderBuffer;
    type IntoIter = IntoIter;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self.rbos)
    }
}

impl RenderBuffer {
    /// Wrapper for `glBindRenderBuffer(...)`
    #[inline]
    pub fn bind(&self) {
        unsafe {
            gl::BindRenderbuffer(gl::RENDERBUFFER, self.rbo);
        }
    }

    /// Wrapper for `glBindRenderBuffer(...)`
    #[inline]
    pub fn unbind() {
        unsafe {
            gl::BindRenderbuffer(gl::RENDERBUFFER, 0);
        }
    }

    /// Wrapper for `glRenderbufferStorage(...)`
    #[inline]
    pub fn storage(format: RenderBufferFormat, (width, height): (u32, u32)) {
        unsafe {
            gl::RenderbufferStorage(
                gl::RENDERBUFFER,
                format.to_gl_format(),
                width as _,
                height as _,
            );
        }
    }
}
