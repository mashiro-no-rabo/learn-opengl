use glfw::Context;
use glfw::{Action, Key, OpenGlProfileHint, WindowHint, WindowMode};
use nalgebra_glm as glm;
use std::ffi::c_void;
use std::mem;

use strugl::{deg_to_rad, Matrix4, ShaderProgram};

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
layout (location = 1) in vec2 aTexCoord;

out vec2 TexCoord;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main()
{
    gl_Position = projection * view * model * vec4(aPos, 1.0);
    TexCoord = aTexCoord;
}";

    // Fragment Shader
    let fs_code = "
#version 460 core
out vec4 FragColor;

in vec2 TexCoord;

uniform sampler2D texture1;
uniform sampler2D texture2;

uniform float mixValue;

void main()
{
    FragColor = mix(texture(texture1, TexCoord), texture(texture2, TexCoord), mixValue);
}";

    // Shader Program
    let sp = unsafe { ShaderProgram::from_str(vs_code, fs_code) };

    // Vertex Data
    let vao = unsafe {
      let mut vertices: Vec<f32> = vec![];
      // position (xyz), texture coord (xy)
      vertices.append(&mut vec![-0.5, -0.5, -0.5, 0.0, 0.0]);
      vertices.append(&mut vec![0.5, -0.5, -0.5, 1.0, 0.0]);
      vertices.append(&mut vec![0.5, 0.5, -0.5, 1.0, 1.0]);
      vertices.append(&mut vec![0.5, 0.5, -0.5, 1.0, 1.0]);
      vertices.append(&mut vec![-0.5, 0.5, -0.5, 0.0, 1.0]);
      vertices.append(&mut vec![-0.5, -0.5, -0.5, 0.0, 0.0]);
      vertices.append(&mut vec![-0.5, -0.5, 0.5, 0.0, 0.0]);
      vertices.append(&mut vec![0.5, -0.5, 0.5, 1.0, 0.0]);
      vertices.append(&mut vec![0.5, 0.5, 0.5, 1.0, 1.0]);
      vertices.append(&mut vec![0.5, 0.5, 0.5, 1.0, 1.0]);
      vertices.append(&mut vec![-0.5, 0.5, 0.5, 0.0, 1.0]);
      vertices.append(&mut vec![-0.5, -0.5, 0.5, 0.0, 0.0]);
      vertices.append(&mut vec![-0.5, 0.5, 0.5, 1.0, 0.0]);
      vertices.append(&mut vec![-0.5, 0.5, -0.5, 1.0, 1.0]);
      vertices.append(&mut vec![-0.5, -0.5, -0.5, 0.0, 1.0]);
      vertices.append(&mut vec![-0.5, -0.5, -0.5, 0.0, 1.0]);
      vertices.append(&mut vec![-0.5, -0.5, 0.5, 0.0, 0.0]);
      vertices.append(&mut vec![-0.5, 0.5, 0.5, 1.0, 0.0]);
      vertices.append(&mut vec![0.5, 0.5, 0.5, 1.0, 0.0]);
      vertices.append(&mut vec![0.5, 0.5, -0.5, 1.0, 1.0]);
      vertices.append(&mut vec![0.5, -0.5, -0.5, 0.0, 1.0]);
      vertices.append(&mut vec![0.5, -0.5, -0.5, 0.0, 1.0]);
      vertices.append(&mut vec![0.5, -0.5, 0.5, 0.0, 0.0]);
      vertices.append(&mut vec![0.5, 0.5, 0.5, 1.0, 0.0]);
      vertices.append(&mut vec![-0.5, -0.5, -0.5, 0.0, 1.0]);
      vertices.append(&mut vec![0.5, -0.5, -0.5, 1.0, 1.0]);
      vertices.append(&mut vec![0.5, -0.5, 0.5, 1.0, 0.0]);
      vertices.append(&mut vec![0.5, -0.5, 0.5, 1.0, 0.0]);
      vertices.append(&mut vec![-0.5, -0.5, 0.5, 0.0, 0.0]);
      vertices.append(&mut vec![-0.5, -0.5, -0.5, 0.0, 1.0]);
      vertices.append(&mut vec![-0.5, 0.5, -0.5, 0.0, 1.0]);
      vertices.append(&mut vec![0.5, 0.5, -0.5, 1.0, 1.0]);
      vertices.append(&mut vec![0.5, 0.5, 0.5, 1.0, 0.0]);
      vertices.append(&mut vec![0.5, 0.5, 0.5, 1.0, 0.0]);
      vertices.append(&mut vec![-0.5, 0.5, 0.5, 0.0, 0.0]);
      vertices.append(&mut vec![-0.5, 0.5, -0.5, 0.0, 1.0]);

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
        5 * mem::size_of::<f32>() as i32,
        (0 * mem::size_of::<f32>()) as *const c_void,
      );
      gl::EnableVertexAttribArray(0);

      gl::VertexAttribPointer(
        1,
        2,
        gl::FLOAT,
        gl::FALSE,
        5 * mem::size_of::<f32>() as i32,
        (3 * mem::size_of::<f32>()) as *const c_void,
      );
      gl::EnableVertexAttribArray(1);

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
        .flipv()
        .into_rgb();

      let (width, height) = img.dimensions();

      gl::TexImage2D(
        gl::TEXTURE_2D,
        0,
        gl::RGB as i32,
        width as i32,
        height as i32,
        0,
        gl::RGB,
        gl::UNSIGNED_BYTE,
        img.into_raw().as_ptr() as *const c_void,
      );
      gl::GenerateMipmap(gl::TEXTURE_2D);

      tex
    };

    let tex2 = unsafe {
      let mut tex = 0;
      gl::GenTextures(1, &mut tex);
      gl::BindTexture(gl::TEXTURE_2D, tex);

      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

      let img = image::open("resources/textures/awesomeface.png")
        .expect("failed to load texture image")
        .flipv()
        .into_rgba();

      let (width, height) = img.dimensions();

      gl::TexImage2D(
        gl::TEXTURE_2D,
        0,
        gl::RGB as i32,
        width as i32,
        height as i32,
        0,
        gl::RGBA,
        gl::UNSIGNED_BYTE,
        img.into_raw().as_ptr() as *const c_void,
      );
      gl::GenerateMipmap(gl::TEXTURE_2D);

      tex
    };

    if wireframe_mode {
      unsafe {
        gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
      }
    }

    // Bind textures
    unsafe {
      sp.use_program();
      sp.set_uniform_value("texture1", 0);
      sp.set_uniform_value("texture2", 1);
    }

    // Transformations
    let base_model = glm::rotate_x(&glm::Mat4::identity(), deg_to_rad(-55.0));
    let view: Matrix4 = glm::translate(&glm::Mat4::identity(), &glm::vec3(0.0, 0.0, -3.0)).into();
    let projection: Matrix4 = glm::perspective_fov(deg_to_rad(45.0), 800.0, 600.0, 0.1, 100.0).into();

    // Interaction
    let mut mix_value = 0.2f32;

    // Loop
    while !window.should_close() {
      glfw.poll_events();
      for (_, event) in glfw::flush_messages(&events) {
        match event {
          glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
            gl::Viewport(0, 0, width, height);
          },
          glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
          glfw::WindowEvent::Key(Key::Up, _, Action::Press, _) => mix_value = (mix_value + 0.1).min(1.0),
          glfw::WindowEvent::Key(Key::Down, _, Action::Press, _) => mix_value = (mix_value - 0.1).max(0.0),
          _ => {}
        }
      }

      unsafe {
        gl::ClearColor(0.2, 0.3, 0.3, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);

        sp.use_program();
        sp.set_uniform_value("mixValue", mix_value);

        let model: Matrix4 = glm::rotate(
          &base_model,
          glfw.get_time() as f32 * deg_to_rad(50.0),
          &glm::vec3(0.5, 1.0, 0.0),
        )
        .into();

        sp.set_uniform_value("model", model);
        sp.set_uniform_value("view", view);
        sp.set_uniform_value("projection", projection);

        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, tex);
        gl::ActiveTexture(gl::TEXTURE1);
        gl::BindTexture(gl::TEXTURE_2D, tex2);

        gl::BindVertexArray(vao);
        gl::DrawArrays(gl::TRIANGLES, 0, 36);
      }

      window.swap_buffers();
    }
  } else {
    panic!("failed to create GLFW window");
  }
}
