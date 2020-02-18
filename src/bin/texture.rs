use glfw::Context;
use glfw::{Action, Key, OpenGlProfileHint, WindowHint, WindowMode};
use std::ffi::c_void;
use std::mem;

use learn_opengl::ShaderProgram;

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

    unsafe {
      gl::Enable(gl::BLEND);
      gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }

    // Vertex Shader
    let vs_code = "
#version 460 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aColor;
layout (location = 2) in vec2 aTexCoord;

out vec3 ourColor;
out vec2 TexCoord;

void main()
{
    gl_Position = vec4(aPos, 1.0);
    ourColor = aColor;
    TexCoord = aTexCoord;
}";

    // Fragment Shader
    let fs_code = "
#version 460 core
out vec4 FragColor;

in vec3 ourColor;
in vec2 TexCoord;

uniform sampler2D texture1;

void main()
{
    FragColor = texture(texture1, TexCoord);
}";

    // Shader Program
    let sp = unsafe { ShaderProgram::from_str(vs_code, fs_code) };

    // Vertex Data
    let vao = unsafe {
      let mut vertices: Vec<f32> = vec![];
      // position (xyz), color (rgb), texture coord (xy)
      vertices.append(&mut vec![0.5, 0.5, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0]);
      vertices.append(&mut vec![0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0]);
      vertices.append(&mut vec![-0.5, -0.5, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0]);
      vertices.append(&mut vec![-0.5, 0.5, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0]);

      let indices = vec![0, 1, 3, 1, 2, 3];

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
        8 * mem::size_of::<f32>() as i32,
        0 as *const c_void,
      );
      gl::EnableVertexAttribArray(0);

      gl::VertexAttribPointer(
        1,
        3,
        gl::FLOAT,
        gl::FALSE,
        8 * mem::size_of::<f32>() as i32,
        3 as *const c_void,
      );
      gl::EnableVertexAttribArray(1);

      gl::VertexAttribPointer(
        2,
        2,
        gl::FLOAT,
        gl::FALSE,
        8 * mem::size_of::<f32>() as i32,
        6 as *const c_void,
      );
      gl::EnableVertexAttribArray(2);

      let mut ebo = 0;
      gl::GenBuffers(1, &mut ebo);

      gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
      gl::BufferData(
        gl::ELEMENT_ARRAY_BUFFER,
        (mem::size_of::<f32>() * vertices.len()) as isize,
        indices.as_ptr() as *const c_void,
        gl::STATIC_DRAW,
      );

      vao
    };

    // Texture
    let tex = unsafe {
      let mut tex = 0;
      gl::GenTextures(1, &mut tex);
      gl::BindTexture(gl::TEXTURE_2D, tex);

      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

      let img = image::open("resources/textures/container.jpg")
        .expect("failed to load texture image")
        .into_rgb();

      let (width, height) = img.dimensions();
      let img_data = img.into_raw();

      gl::TexImage2D(
        gl::TEXTURE_2D,
        0,
        gl::RGB as i32,
        width as i32,
        height as i32,
        0,
        gl::RGB,
        gl::UNSIGNED_BYTE,
        img_data.as_ptr() as *const c_void,
      );
      gl::GenerateMipmap(gl::TEXTURE_2D);

      tex
    };

    if wireframe_mode {
      unsafe {
        gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
      }
    }

    unsafe {
      sp.use_program();
      sp.set_uniform_value("texture1", 0);
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

        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, tex);

        gl::BindVertexArray(vao);
        gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, 0 as *const c_void);
      }

      window.swap_buffers();
    }
  } else {
    panic!("failed to create GLFW window");
  }
}
