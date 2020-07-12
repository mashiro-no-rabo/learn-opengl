use glfw::Context;
use glfw::{OpenGlProfileHint, WindowHint, WindowMode};
use nalgebra_glm as glm;
use std::ffi::c_void;
use std::mem;

use strugl::{deg_to_rad, Matrix4, ShaderProgram, Vec3};

const INIT_WIDTH: u32 = 800;
const INIT_HEIGHT: u32 = 600;

fn main() {
  let mut wireframe_mode = false;
  for arg in std::env::args() {
    if &arg == "--wireframe" {
      wireframe_mode = true;
    }
  }

  // Create Window
  let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
  glfw.window_hint(WindowHint::ContextVersion(4, 6));
  glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));

  if let Some((mut window, events)) =
    glfw.create_window(INIT_WIDTH, INIT_HEIGHT, "Rust-LearnOpenGL", WindowMode::Windowed)
  {
    window.make_current();
    window.set_framebuffer_size_polling(true);
    window.set_key_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_cursor_mode(glfw::CursorMode::Disabled);
    window.set_scroll_polling(true);

    // Load OpenGL functions
    gl_loader::init_gl();
    gl::load_with(|symbol| gl_loader::get_proc_address(symbol) as *const _);

    unsafe {
      gl::Viewport(0, 0, INIT_WIDTH as i32, INIT_HEIGHT as i32);
    }

    // Blending
    unsafe {
      gl::Enable(gl::BLEND);
      gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

      gl::Enable(gl::DEPTH_TEST);
    }

    // Wireframe if enabled
    if wireframe_mode {
      unsafe {
        gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
      }
    }

    // Vertex Shader
    let vs_code = "
#version 460 core
layout (location = 0) in vec3 aPos;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main()
{
  gl_Position = projection * view * model * vec4(aPos, 1.0);
}";

    // Fragment Shader
    let fs_code = "
#version 460 core
out vec4 FragColor;

uniform vec3 objectColor;
uniform vec3 lightColor;

void main()
{
  FragColor = vec4(lightColor * objectColor, 1.0);
}";

    let light_fs_code: &str = "
#version 460 core
out vec4 FragColor;

void main()
{
  FragColor = vec4(1.0);
}";

    // Shader Program
    let sp = unsafe { ShaderProgram::from_str(vs_code, fs_code) };
    let light_sp = unsafe { ShaderProgram::from_str(vs_code, light_fs_code) };

    // Vertex Data
    let (cube_vao, light_vao) = unsafe {
      let mut vertices: Vec<f32> = vec![];
      // position (xyz)
      vertices.append(&mut vec![
        -0.5, -0.5, -0.5, 0.5, -0.5, -0.5, 0.5, 0.5, -0.5, 0.5, 0.5, -0.5, -0.5, 0.5, -0.5, -0.5, -0.5, -0.5, -0.5,
        -0.5, 0.5, 0.5, -0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, -0.5, 0.5, 0.5, -0.5, -0.5, 0.5, -0.5, 0.5, 0.5, -0.5,
        0.5, -0.5, -0.5, -0.5, -0.5, -0.5, -0.5, -0.5, -0.5, -0.5, 0.5, -0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, -0.5,
        0.5, -0.5, -0.5, 0.5, -0.5, -0.5, 0.5, -0.5, 0.5, 0.5, 0.5, 0.5, -0.5, -0.5, -0.5, 0.5, -0.5, -0.5, 0.5, -0.5,
        0.5, 0.5, -0.5, 0.5, -0.5, -0.5, 0.5, -0.5, -0.5, -0.5, -0.5, 0.5, -0.5, 0.5, 0.5, -0.5, 0.5, 0.5, 0.5, 0.5,
        0.5, 0.5, -0.5, 0.5, 0.5, -0.5, 0.5, -0.5,
      ]);

      let mut vbo = 0;
      gl::GenBuffers(1, &mut vbo);

      let mut vaos = vec![0, 0];
      gl::GenVertexArrays(2, vaos.as_mut_ptr());

      // Cube
      gl::BindVertexArray(vaos[0]);
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
        3 * mem::size_of::<f32>() as i32,
        (0 * mem::size_of::<f32>()) as *const c_void,
      );
      gl::EnableVertexAttribArray(0);

      // Light
      gl::BindVertexArray(vaos[1]);
      gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

      gl::VertexAttribPointer(
        0,
        3,
        gl::FLOAT,
        gl::FALSE,
        3 * mem::size_of::<f32>() as i32,
        (0 * mem::size_of::<f32>()) as *const c_void,
      );
      gl::EnableVertexAttribArray(0);

      (vaos[0], vaos[1])
    };

    // Transformations
    let base_model = glm::rotate_x(&glm::Mat4::identity(), deg_to_rad(-55.0));

    // Camera
    let mut camera_pos = glm::vec3(0.0, 0.0, 3.0);
    let mut camera_front = glm::vec3(0.0, 0.0, -1.0);
    let camera_up = glm::vec3(0.0, 1.0, 0.0);

    // Interaction
    let mut mix_value = 0.2f32;

    let camera_speed = 2.532;
    let mut last_time = 0.0;

    let mut last_mouse = None;
    let mouse_sensitivity = 0.05;
    let mut pitch_deg = 0.0f32;
    let mut yaw_deg = -90.0f32;

    let mut fov = 45.0;

    // Light Source
    let light_pos = glm::vec3(1.2f32, 1.0, 2.0);
    let light_model = glm::Mat4::identity();
    let light_model = glm::translate(&light_model, &light_pos);
    let light_model: Matrix4 = glm::scale(&light_model, &glm::vec3(0.2f32, 0.2, 0.2)).into();

    // Colors
    let obj_color: Vec3 = glm::vec3(1.0f32, 0.5, 0.31).into();
    let light_color: Vec3 = glm::vec3(1.0f32, 1.0, 1.0).into();

    // Loop
    while !window.should_close() {
      let current = glfw.get_time();
      let delta_time = (current - last_time) as f32;
      last_time = current;

      let camera_movement = camera_speed * delta_time;

      glfw.poll_events();
      for (_, event) in glfw::flush_messages(&events) {
        use glfw::{Action::*, Key::*, WindowEvent::*};

        match event {
          FramebufferSize(width, height) => unsafe {
            gl::Viewport(0, 0, width, height);
          },
          Key(Escape, _, Press, _) => window.set_should_close(true),
          Key(Up, _, Press, _) => mix_value = (mix_value + 0.1).min(1.0),
          Key(Down, _, Press, _) => mix_value = (mix_value - 0.1).max(0.0),
          Key(W, _, Repeat, _) => camera_pos += camera_movement * camera_front,
          Key(S, _, Repeat, _) => camera_pos -= camera_movement * camera_front,
          Key(A, _, Repeat, _) => {
            camera_pos -= camera_movement * glm::normalize(&glm::cross(&camera_front, &camera_up))
          }
          Key(D, _, Repeat, _) => {
            camera_pos += camera_movement * glm::normalize(&glm::cross(&camera_front, &camera_up))
          }
          CursorPos(x, y) => {
            if let Some((last_x, last_y)) = last_mouse {
              let offset_x = (x - last_x) as f32 * mouse_sensitivity;
              let offset_y = (y - last_y) as f32 * mouse_sensitivity;
              pitch_deg = (pitch_deg - offset_y).min(60.0).max(-60.0);
              yaw_deg = yaw_deg + offset_x;

              let dir_x = (deg_to_rad(yaw_deg) * deg_to_rad(pitch_deg).cos()).cos();
              let dir_y = deg_to_rad(pitch_deg).sin();
              let dir_z = (deg_to_rad(yaw_deg) * deg_to_rad(pitch_deg).cos()).sin();

              camera_front = glm::normalize(&glm::vec3(dir_x, dir_y, dir_z));
            }
            last_mouse = Some((x, y));
          }
          Scroll(_x, y) => {
            fov = (fov - y as f32).min(45.0).max(1.0);
          }
          _ => {}
        }
      }

      unsafe {
        gl::ClearColor(0.2, 0.3, 0.3, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

        sp.use_program();
        sp.set_uniform_value("objectColor", obj_color);
        sp.set_uniform_value("lightColor", light_color);

        let model: Matrix4 = glm::rotate(
          &base_model,
          glfw.get_time() as f32 * deg_to_rad(50.0),
          &glm::vec3(0.5, 1.0, 0.0),
        )
        .into();

        let view: Matrix4 = glm::look_at(&camera_pos, &(camera_pos + camera_front), &camera_up).into();
        let projection: Matrix4 = glm::perspective_fov(deg_to_rad(fov), 800.0, 600.0, 0.1, 100.0).into();

        sp.set_uniform_value("model", model);
        sp.set_uniform_value("view", view);
        sp.set_uniform_value("projection", projection);

        gl::BindVertexArray(cube_vao);
        gl::DrawArrays(gl::TRIANGLES, 0, 36);

        light_sp.use_program();
        light_sp.set_uniform_value("model", light_model);
        light_sp.set_uniform_value("view", view);
        light_sp.set_uniform_value("projection", projection);

        gl::BindVertexArray(light_vao);
        gl::DrawArrays(gl::TRIANGLES, 0, 36);
      }

      window.swap_buffers();
    }
  } else {
    panic!("failed to create GLFW window");
  }
}
