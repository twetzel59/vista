//! # Vista
//! A simple abstraction over OpenGL for type safety and RAII.
//! Please note that Vista does not abstract away the state machine,
//! its only intent is to make a safe API for Rust use of OpenGL.
//! Also, only a very small, basic, portion of OpenGL will be wrapped —
//! the parts that I am using in my simple games and scenes.
//! This will include primarily Vertex Arrays, Vertex ARRAY_BUFFERs and
//! ELEMENT_ARRAY_BUFFERs, 2D Textures, and Vertex and Fragment shaders.


extern crate gl;

//pub use gl_object::GlObject;

pub mod buffers;
pub mod gl_object;
pub mod vertex_arrays;
