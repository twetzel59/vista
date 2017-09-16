use std::mem;
use gl;
use gl::types::*;
use gl_object::{GlEnum, GlObject};

/// A VBO, or vertex buffer object
pub struct Buffer {
    id: GLuint,
    kind: Kind,
}

impl Buffer {
    /// Creates a new VBO
    pub fn new(kind: Kind) -> Buffer {
        let mut id = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }

        Buffer {
            id,
            kind,
        }
    }

    /// Binds the VBO
    /// The binding is to the target that `kind` represents
    /// upon construction. Some targets include GL_ARRAY_BUFFER
    /// and GL_ELEMENT_ARRAY_BUFFER.
    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(self.kind.into(), self.id);
        }
    }

    /// Unbinds the VBO
    /// Only the target in `kind` from construction is affected.
    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(self.kind.into(), 0);
        }
    }

    /// Reallocate and add data to the buffer
    /// # Parameters
    /// `data` is simply the slice of data to send to the GPU.
    /// `usage` is only a hint to the driver how the data will
    /// be used. A good choice could improve optimizations.
    pub fn data(&mut self, data: &[GLfloat], usage: Usage) {
        unsafe {
            gl::NamedBufferData(self.id,
                                (data.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                                data.as_ptr() as *const _,
                                usage.into());
        }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}

impl GlObject for Buffer {}

/// The targets supported for VBOs
#[derive(Clone, Copy)]
pub enum Kind {
    ArrayBuffer,
}

impl From<Kind> for GLenum {
    fn from(kind: Kind) -> Self {
        match kind {
            Kind::ArrayBuffer => gl::ARRAY_BUFFER,
        }
    }
}

impl GlEnum for Kind {}

/// Hints as to the memory usage pattern of the VBO
#[derive(Clone, Copy)]
pub enum Usage {
    DynamicDraw,
    StaticDraw,
    StreamDraw,
}

impl From<Usage> for GLenum {
    fn from(usage: Usage) -> Self {
        match usage {
            Usage::DynamicDraw => gl::DYNAMIC_DRAW,
            Usage::StaticDraw => gl::STATIC_DRAW,
            Usage::StreamDraw => gl::STREAM_DRAW,
        }
    }
}

impl GlEnum for Usage {}
