use gl;
use gl::types::*;
use gl_object::GlObject;

/// A Vertex Array, or VAO
/// It stores state related to vertex data.
pub struct VertexArray {
    id: GLuint,
}

impl VertexArray {
    /// Creates a new Vertex Array
    pub fn new() -> VertexArray {
        let mut id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }

        VertexArray {
            id,
        }
    }

    /// Binds the vertex array
    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    /// Unbinds the vertex array
    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.id);
        }
    }
}

impl GlObject for VertexArray {}
