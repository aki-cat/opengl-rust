use crate::{Array, Buffer, Context};

/// Vertex Object
pub struct Vertex {
    array: Array,
    buffers: Vec<Buffer>,
}

impl Vertex {
    /// Create a new vertex object.
    #[inline]
    pub fn new(context: &Context) -> Self {
        let array = unsafe { context.new_array() };
        let buffers = Vec::new();
        Self { array, buffers }
    }

    /// Create a new buffer then initialize it by func `init` and attach it to this vertex object.
    ///
    /// # Note
    ///
    /// The `init` function will be called in the context of array and accept the buffer as argument.
    #[inline]
    pub fn new_buffer<F: FnOnce(&mut Buffer)>(&mut self, context: &Context, init: F) {
        let mut buffer = unsafe { context.new_buffer() };
        self.array.bind();
        init(&mut buffer);
        self.buffers.push(buffer);
    }

    /// Bind its array to the current context.
    #[inline]
    pub fn using(&self) {
        self.array.bind();
    }
}
