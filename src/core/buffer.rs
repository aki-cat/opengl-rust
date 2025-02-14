use std::ops::{Index, IndexMut};

use gl::types::GLuint;

use super::{enums::*, GlTypeT};

/// Vertex Buffer Object
pub struct Buffer {
    buffer: GLuint,
}

impl Buffer {
    pub(super) fn new() -> Self {
        let mut buffer = 0;
        unsafe {
            gl::GenBuffers(1, &mut buffer);
        }
        Self { buffer }
    }
}

impl Drop for Buffer {
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
    pub fn iter(&self) -> impl Iterator<Item = &Buffer> {
        self.buffers.iter()
    }

    /// Return an iterator of the buffers.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Buffer> {
        self.buffers.iter_mut()
    }
}

pub struct IntoIter {
    buffers: Vec<Option<Buffer>>,
    index: usize,
}

impl IntoIter {
    fn new(buffers: Vec<Buffer>) -> Self {
        let buffers = buffers.into_iter().map(|buffer| Some(buffer)).collect();
        Self { buffers, index: 0 }
    }
}

impl Iterator for IntoIter {
    type Item = Buffer;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.buffers.get_mut(self.index)?.take();
        self.index += 1;
        result
    }
}

impl IntoIterator for Buffers {
    type Item = Buffer;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self.buffers)
    }
}

impl Index<usize> for Buffers {
    type Output = Buffer;

    fn index(&self, index: usize) -> &Self::Output {
        &self.buffers[index]
    }
}

impl IndexMut<usize> for Buffers {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.buffers[index]
    }
}

impl Buffer {
    /// Bind this buffer in the current context.
    pub fn bind(&self, target: Target) {
        unsafe {
            gl::BindBuffer(target.to_gl_target(), self.buffer);
        }
    }

    /// Unbind this buffer in the current context.
    pub fn unbind(target: Target) {
        unsafe {
            gl::BindBuffer(target.to_gl_target(), 0);
        }
    }

    /// Upload `data` to this buffer.
    pub fn data<'a, 'b: 'a, T: Copy>(&'a self, target: Target, data: &'b [T], usage: Usage) {
        unsafe {
            gl::BufferData(
                target.to_gl_target(),
                (data.len() * size_of::<T>()) as _,
                data.as_ptr() as *const _,
                usage.to_gl_usage(),
            );
        }
    }

    /// Mark the vertex attribute of buffer data.
    #[allow(private_bounds)]
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
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GlType {
    u8,
    i8,
    u16,
    i16,
    u32,
    i32,
    f32,
    f64,
}

impl GlType {
    pub(super) const fn size(self) -> usize {
        match self {
            GlType::u8 => std::mem::size_of::<u8>(),
            GlType::i8 => std::mem::size_of::<i8>(),
            GlType::u16 => std::mem::size_of::<u16>(),
            GlType::i16 => std::mem::size_of::<i16>(),
            GlType::u32 => std::mem::size_of::<u32>(),
            GlType::i32 => std::mem::size_of::<i32>(),
            GlType::f32 => std::mem::size_of::<f32>(),
            GlType::f64 => std::mem::size_of::<f64>(),
        }
    }

    pub(super) const fn to_gl_type(self) -> GLuint {
        match self {
            GlType::u8 => gl::UNSIGNED_BYTE,
            GlType::i8 => gl::BYTE,
            GlType::u16 => gl::UNSIGNED_SHORT,
            GlType::i16 => gl::SHORT,
            GlType::u32 => gl::UNSIGNED_INT,
            GlType::i32 => gl::INT,
            GlType::f32 => gl::FLOAT,
            GlType::f64 => gl::DOUBLE,
        }
    }
}
