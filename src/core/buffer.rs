use std::ops::{Index, IndexMut};

use gl::types::GLuint;

use super::{enums::*, GlTypeT};

/// Vertex Buffer Object
pub struct Buffer {
    buffer: GLuint,
}

impl Buffer {
    #[inline]
    pub(super) fn new() -> Self {
        let mut buffer = 0;
        unsafe {
            gl::GenBuffers(1, &mut buffer);
        }
        Self { buffer }
    }
}

impl Drop for Buffer {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.buffer);
        }
    }
}

/// Vertex Buffer Objects
pub struct Buffers {
    buffers: Vec<Buffer>,
}

impl Buffers {
    #[inline]
    pub(super) fn new(count: usize) -> Self {
        assert!(count > 0, "The number of buffers must be greater than 0");
        let mut buffers = Vec::with_capacity(count);
        unsafe {
            gl::GenBuffers(count as _, buffers.as_mut_ptr());
        }
        let buffers = buffers
            .into_iter()
            .map(|buffer| Buffer { buffer })
            .collect();
        Self { buffers }
    }

    /// Return an iterator of the buffers.
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &Buffer> {
        self.buffers.iter()
    }

    /// Return an iterator of the buffers.
    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Buffer> {
        self.buffers.iter_mut()
    }
}

pub struct IntoIter {
    buffers: Vec<Option<Buffer>>,
    index: usize,
}

impl IntoIter {
    #[inline]
    fn new(buffers: Vec<Buffer>) -> Self {
        let buffers = buffers.into_iter().map(|buffer| Some(buffer)).collect();
        Self { buffers, index: 0 }
    }
}

impl Iterator for IntoIter {
    type Item = Buffer;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let result = self.buffers.get_mut(self.index)?.take();
        self.index += 1;
        result
    }
}

impl IntoIterator for Buffers {
    type Item = Buffer;
    type IntoIter = IntoIter;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self.buffers)
    }
}

impl Index<usize> for Buffers {
    type Output = Buffer;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.buffers[index]
    }
}

impl IndexMut<usize> for Buffers {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.buffers[index]
    }
}

impl Buffer {
    /// Bind this buffer in the current context.
    #[inline]
    pub fn bind(&self, target: Target) {
        unsafe {
            gl::BindBuffer(target.to_gl_target(), self.buffer);
        }
    }

    /// Unbind this buffer in the current context.
    #[inline]
    pub fn unbind(target: Target) {
        unsafe {
            gl::BindBuffer(target.to_gl_target(), 0);
        }
    }

    /// Mark the vertex attribute of buffer data.
    #[allow(private_bounds)]
    #[inline]
    pub unsafe fn mark<T: Copy + GlTypeT>(
        index: usize,
        count: usize,
        should_normlized: bool,
        stride: usize,
        offset: usize,
    ) {
        gl::VertexAttribPointer(
            index as _,
            count as _,
            T::to_gl_type(),
            if should_normlized {
                gl::TRUE
            } else {
                gl::FALSE
            },
            stride as _,
            offset as _,
        );
        gl::EnableVertexAttribArray(index as _);
    }

    /// Mark the vertex attribute of buffer data by `composition`.
    ///
    /// The normalized is false by default.
    #[inline]
    pub fn gen_mark(composition: &[(GlType, usize)]) {
        let mut current_pos = 0;
        let stride: usize = composition
            .iter()
            .map(|&(ty, count)| ty.size() * count)
            .sum();
        for (index, &(ty, count)) in composition.iter().enumerate() {
            unsafe {
                gl::VertexAttribPointer(
                    index as _,
                    count as _,
                    ty.to_gl_type() as _,
                    false as _,
                    stride as _,
                    current_pos as _,
                );
                gl::EnableVertexAttribArray(index as _);
            }
            current_pos += ty.size() * count;
        }
    }

    /// Mark the vertex attribute of buffer data by `composition` with normalized.
    #[inline]
    pub fn gen_mark_with_normalized(composition: &[(GlType, usize, bool)]) {
        let mut current_pos = 0;
        let stride: usize = composition
            .iter()
            .map(|&(ty, count, _)| ty.size() * count)
            .sum();
        for (index, &(ty, count, normalized)) in composition.iter().enumerate() {
            unsafe {
                gl::VertexAttribPointer(
                    index as _,
                    count as _,
                    ty.to_gl_type() as _,
                    normalized as _,
                    stride as _,
                    current_pos as _,
                );
                gl::EnableVertexAttribArray(index as _);
            }
            current_pos += ty.size() * count;
        }
    }

    /// Upload `data` to this buffer.
    #[inline]
    pub fn data<'a, 'b: 'a, T: Copy>(&'a self, data: &'b [T], usage: Usage) {
        unsafe {
            gl::NamedBufferData(
                self.buffer,
                (data.len() * size_of::<T>()) as _,
                data.as_ptr() as _,
                usage.to_gl_usage(),
            );
        }
    }

    pub unsafe fn sub_data<'a, 'b: 'a, T: Copy>(&'a self, offset: usize, data: &'b [T]) {
        unsafe {
            gl::NamedBufferSubData(
                self.buffer,
                offset as _,
                (data.len() * size_of::<T>()) as _,
                data.as_ptr() as _,
            );
        }
    }
}
