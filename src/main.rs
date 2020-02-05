use glfw::Context;
use glfw::{Action, Key, OpenGlProfileHint, WindowHint, WindowMode};
use std::ffi::CString;
use std::mem;
use std::ptr::{null, null_mut};

fn main() {
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

    // Vertex Data
    let vertices: Vec<f32> = vec![-0.5, 0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];
    let mut vbo = 0;
    unsafe {
      gl::GenBuffers(1, &mut vbo);
      gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
      gl::BufferData(
        gl::ARRAY_BUFFER,
        (mem::size_of::<f32>() * vertices.len()) as isize,
        vertices.as_ptr() as *const std::ffi::c_void,
        gl::STATIC_DRAW,
      );
    }

    // Vertex Shader
    let vs_code = CString::new(
      "
#version 460 core
layout (location=0) in vec3 aPos;

void main() {
  gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
}",
    )
    .unwrap();

    unsafe {
      let vs_id = gl::CreateShader(gl::VERTEX_SHADER);
      gl::ShaderSource(vs_id, 1, &vs_code.as_c_str().as_ptr(), null());
      gl::CompileShader(vs_id);

      let mut success = 0;
      let mut log = [0; 512];

      gl::GetShaderiv(vs_id, gl::COMPILE_STATUS, &mut success);
      if success as u8 != gl::TRUE {
        gl::GetShaderInfoLog(vs_id, 512, null_mut(), log.as_mut_ptr());
        panic!(
          "Vertex Shader failed to compile:\n{}",
          String::from_utf8(log.into_iter().map(|x| *x as u8).collect()).unwrap()
        );
      }
    }

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
      }

      window.swap_buffers();
    }
  } else {
    panic!("failed to create GLFW window");
  }
}
