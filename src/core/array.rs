use std::ops::{Index, IndexMut};

use gl::types::GLuint;

/// Vertex Array Object
pub struct Array {
    array: GLuint,
}

impl Array {
    pub(super) fn new() -> Self {
        let mut array = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut array);
        }
        Self { array }
    }
}

impl Drop for Array {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.array);
        }
    }
}

/// Vertex Array Objects
pub struct Arrays {
    arrays: Vec<Array>,
}

impl Arrays {
    pub(super) fn new(count: usize) -> Self {
        let mut arrays = Vec::with_capacity(count);
        unsafe {
            gl::GenVertexArrays(count as i32, arrays.as_mut_ptr());
        }
        let arrays = arrays.into_iter().map(|array| Array { array }).collect();
        Self { arrays }
    }

    /// Return an iterator of the arrays.
    pub fn iter(&self) -> impl Iterator<Item = &Array> {
        self.arrays.iter()
    }

    /// Return an iterator of the arrays.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Array> {
        self.arrays.iter_mut()
    }

    /// Return the number of arrays.
    pub fn count(&self) -> usize {
        self.arrays.len()
    }
}

impl Index<usize> for Arrays {
    type Output = Array;

    fn index(&self, index: usize) -> &Self::Output {
        &self.arrays[index]
    }
}

impl IndexMut<usize> for Arrays {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.arrays[index]
    }
}

pub struct IntoIter {
    arrays: Vec<Option<Array>>,
    index: usize,
}

impl IntoIter {
    fn new(arrays: Vec<Array>) -> Self {
        let arrays = arrays.into_iter().map(|array| Some(array)).collect();
        Self { arrays, index: 0 }
    }
}

impl Iterator for IntoIter {
    type Item = Array;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.arrays.get_mut(self.index)?.take();
        self.index += 1;
        result
    }
}

impl IntoIterator for Arrays {
    type Item = Array;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self.arrays)
    }
}

impl Array {
    /// Bind this array in the current context.
    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.array);
        }
    }

    /// Unbind this array in the current context.
    pub fn unbind() {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}
