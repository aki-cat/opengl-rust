use gl::types::GLuint;

use super::FrameBufferTarget;

pub struct FrameBuffer {
    fbo: GLuint,
}

impl FrameBuffer {
    #[inline]
    pub(super) fn new() -> Self {
        let mut fbo = 0;
        unsafe {
            gl::GenFramebuffers(1, &mut fbo);
        }
        Self { fbo }
    }
}

impl Drop for FrameBuffer {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            gl::DeleteFramebuffers(1, &self.fbo);
        }
    }
}

pub struct FrameBuffers {
    fbos: Vec<FrameBuffer>,
}

impl FrameBuffers {
    #[inline]
    pub(super) fn new(count: usize) -> Self {
        let mut fbos = Vec::with_capacity(count);
        unsafe {
            gl::GenFramebuffers(count as i32, fbos.as_mut_ptr());
        }
        Self {
            fbos: fbos.into_iter().map(|fbo| FrameBuffer { fbo }).collect(),
        }
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &FrameBuffer> {
        self.fbos.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut FrameBuffer> {
        self.fbos.iter_mut()
    }
}

pub struct IntoIter {
    fbos: Vec<Option<FrameBuffer>>,
    index: usize,
}

impl IntoIter {
    #[inline]
    pub fn new(fbos: Vec<FrameBuffer>) -> Self {
        Self {
            fbos: fbos.into_iter().map(|fbo| Some(fbo)).collect(),
            index: 0,
        }
    }
}

impl Iterator for IntoIter {
    type Item = FrameBuffer;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let result = self.fbos.get_mut(self.index)?.take();
        self.index += 1;
        result
    }
}

impl IntoIterator for FrameBuffers {
    type Item = FrameBuffer;

    type IntoIter = IntoIter;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self.fbos)
    }
}

impl FrameBuffer {
    #[inline]
    pub fn bind(&self, target: FrameBufferTarget) {
        unsafe {
            gl::BindFramebuffer(target.to_gl_target(), self.fbo);
        }
    }

    pub fn unbind(target: FrameBufferTarget) {
        unsafe {
            gl::BindFramebuffer(target.to_gl_target(), 0);
        }
    }
}
