use glfw::Context;
use glfw::{Action, Key, OpenGlProfileHint, WindowHint, WindowMode};
use std::ffi::c_void;
use std::mem;

use learn_opengl::{ShaderProgram, UniformValue};

fn main() {
  let mut wireframe_mode = false;
  for arg in std::env::args() {
    if &arg == "--wireframe" {
      wireframe_mode = true;
    }
  }

  let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
  glfw.window_hint(WindowHint::ContextVersion(4, 6));
  glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));

  if let Some((mut window, events)) = glfw.create_window(800, 600, "Rust-LearnOpenGL", WindowMode::Windowed) {
    window.make_current();

    gl_loader::init_gl();
    gl::load_with(|symbol| gl_loader::get_proc_address(symbol) as *const _);

    unsafe {
      gl::Viewport(0, 0, 800, 600);
    }

    window.set_framebuffer_size_polling(true);
    window.set_key_polling(true);

    let mut max_vertex_attributes = 0;
    unsafe {
      gl::GetIntegerv(gl::MAX_VERTEX_ATTRIBS, &mut max_vertex_attributes);
    }
    println!("Max Vertex Attributes: {}", max_vertex_attributes);

    // Vertex Shader
    let vs_code = "
#version 460 core
layout (location=0) in vec3 aPos;
layout (location=1) in vec3 aColor;

uniform float xOffset;

out vec3 vtxColor;

void main() {
  gl_Position = vec4(aPos.x + xOffset, -aPos.y, aPos.z, 1.0);
  vtxColor = aColor;
}";

    // Fragment Shader
    let fs_code = "
#version 460 core
in vec3 vtxColor;
uniform float uAlpha;

out vec4 FragColor;

void main() {
  FragColor = vec4(vtxColor, uAlpha);
}";

    // Shader Program
    let sp = unsafe { ShaderProgram::from_str(vs_code, fs_code) };

    // Vertex Data
    let va_triangle = unsafe {
      let mut vertices: Vec<f32> = vec![];
      vertices.append(&mut vec![0.5, -0.5, 0.0, 1.0, 0.0, 0.0]);
      vertices.append(&mut vec![-0.5, -0.5, 0.0, 0.0, 1.0, 0.0]);
      vertices.append(&mut vec![0.0, 0.5, 0.0, 0.0, 0.0, 1.0]);

      let mut vbo = 0;
      gl::GenBuffers(1, &mut vbo);

      let mut vao = 0;
      gl::GenVertexArrays(1, &mut vao);

      gl::BindVertexArray(vao);
      gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
      gl::BufferData(
        gl::ARRAY_BUFFER,
        (mem::size_of::<f32>() * vertices.len()) as isize,
        vertices.as_ptr() as *const c_void,
        gl::STATIC_DRAW,
      );

      gl::VertexAttribPointer(
        0,
        3,
        gl::FLOAT,
        gl::FALSE,
        6 * mem::size_of::<f32>() as i32,
        0 as *const c_void,
      );
      gl::EnableVertexAttribArray(0);

      gl::VertexAttribPointer(
        1,
        3,
        gl::FLOAT,
        gl::FALSE,
        6 * mem::size_of::<f32>() as i32,
        (3 * mem::size_of::<f32>()) as *const c_void,
      );
      gl::EnableVertexAttribArray(1);

      vao
    };

    if wireframe_mode {
      unsafe {
        gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
      }
    }

    // Loop
    while !window.should_close() {
      glfw.poll_events();
      for (_, event) in glfw::flush_messages(&events) {
        println!("{:?}", event);

        match event {
          glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
            gl::Viewport(0, 0, width, height);
          },
          glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
          _ => {}
        }
      }

      unsafe {
        gl::ClearColor(0.2, 0.3, 0.3, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);

        sp.use_program();
        sp.set_uniform_value("xOffset", 0.3f32);

        // TODO: this doesn't work
        sp.set_uniform_value("uAlpha", 0.2f32);

        gl::BindVertexArray(va_triangle);
        gl::DrawArrays(gl::TRIANGLES, 0, 3);
      }

      window.swap_buffers();
    }
  } else {
    panic!("failed to create GLFW window");
  }
}
