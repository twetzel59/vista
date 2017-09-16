extern crate gl;
extern crate glutin;
extern crate vista;

use gl::types::*;
use glutin::{Api, ContextBuilder, Event, EventsLoop,
             GlContext, GlProfile, GlRequest, GlWindow, VirtualKeyCode,
             WindowBuilder, WindowEvent};
use vista::*;

const VERTICES: [GLfloat; 9] = [-0.5, -0.5, 0.0,
                                 0.5, -0.5, 0.0,
                                 0.0,  0.5, 0.0];

fn main() {
    let mut events_loop = EventsLoop::new();
    let window = WindowBuilder::new()
                    .with_title("luminance_experiment")
                    .with_dimensions(1024, 768);
    let context = ContextBuilder::new()
                    .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
                    .with_gl_profile(GlProfile::Core)
                    .with_vsync(true);
    let gl_window = GlWindow::new(window, context, &events_loop).unwrap();

    unsafe {
        assert!(gl_window.make_current().is_ok());
    }

    gl::load_with(|ptr| gl_window.get_proc_address(ptr) as *const _);

    unsafe {
        println!("OpenGL version {}", String::from_utf8_lossy(
            ::std::ffi::CStr::from_ptr(gl::GetString(gl::VERSION) as *const _)
                .to_bytes().to_vec().as_slice()));
    }

    let vao = vertex_arrays::VertexArray::new();
    vao.bind();

    let mut buffer = buffers::Buffer::new(buffers::Kind::ArrayBuffer);
    buffer.data(&VERTICES, buffers::Usage::StaticDraw);
    buffer.bind();

    //vao.unbind();

    loop {
        let mut done = false;
        events_loop.poll_events(|event| {
            //println!("{:?}", event);
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::Closed => done = true,
                    WindowEvent::KeyboardInput { input, .. } => {
                        if let Some(code) = input.virtual_keycode {
                            match code {
                                VirtualKeyCode::Escape => done = true,
                                _ => {},
                            }
                        }
                    },
                    WindowEvent::Resized(w, h) => gl_window.resize(w, h),
                    _ => (),
                },
                _ => ()
            }
        });

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);


            //vao.bind();

            gl::EnableVertexAttribArray(0);

            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, std::mem::transmute(0usize));

            gl::DrawArrays(gl::TRIANGLES, 0, (VERTICES.len() / 3) as GLsizei);

            gl::DisableVertexAttribArray(0);

            //vao.unbind();
        }

        assert!(gl_window.swap_buffers().is_ok());

        if done {
            break;
        }
    }
}
