/// Trait representing all standard OpenGL objects.
/// Currently, it just requires Drop and has no methods.
pub trait GlObject: Drop {}

/// Trait that allows conversion from Vista enums to `GLenum`s.
/// It requires an implementation of Into `GLenum`.
pub trait GlEnum: Into<::gl::types::GLenum> {}
