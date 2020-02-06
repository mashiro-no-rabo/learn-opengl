use glfw::Context;
use glfw::{Action, Key, OpenGlProfileHint, WindowHint, WindowMode};
use std::ffi::{c_void, CString};
use std::mem;
use std::ptr::{null, null_mut};

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

    let vs = unsafe {
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

      vs_id
    };

    // Fragment Shader
    let fs_code1 = CString::new(
      "
#version 460 core
out vec4 FragColor;

void main() {
  FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
}
",
    )
    .unwrap();

    let fs1 = unsafe {
      let fs_id = gl::CreateShader(gl::FRAGMENT_SHADER);
      gl::ShaderSource(fs_id, 1, &fs_code1.as_c_str().as_ptr(), null());
      gl::CompileShader(fs_id);

      let mut success = 0;
      let mut log = [0; 512];

      gl::GetShaderiv(fs_id, gl::COMPILE_STATUS, &mut success);
      if success as u8 != gl::TRUE {
        gl::GetShaderInfoLog(fs_id, 512, null_mut(), log.as_mut_ptr());
        panic!(
          "Fragment Shader 1 failed to compile:\n{}",
          String::from_utf8(log.into_iter().map(|x| *x as u8).collect()).unwrap()
        );
      }

      fs_id
    };

    let fs_code2 = CString::new(
      "
#version 460 core
out vec4 FragColor;

void main() {
  FragColor = vec4(0.99607f, 0.87450f, 88235f, 1.0f);
}
",
    )
    .unwrap();

    let fs2 = unsafe {
      let fs_id = gl::CreateShader(gl::FRAGMENT_SHADER);
      gl::ShaderSource(fs_id, 1, &fs_code2.as_c_str().as_ptr(), null());
      gl::CompileShader(fs_id);

      let mut success = 0;
      let mut log = [0; 512];

      gl::GetShaderiv(fs_id, gl::COMPILE_STATUS, &mut success);
      if success as u8 != gl::TRUE {
        gl::GetShaderInfoLog(fs_id, 512, null_mut(), log.as_mut_ptr());
        panic!(
          "Fragment Shader 2 failed to compile:\n{}",
          String::from_utf8(log.into_iter().map(|x| *x as u8).collect()).unwrap()
        );
      }

      fs_id
    };

    // Shader Program
    let sp1 = unsafe {
      let sp = gl::CreateProgram();
      gl::AttachShader(sp, vs);
      gl::AttachShader(sp, fs1);
      gl::LinkProgram(sp);

      let mut success = 0;
      let mut log = [0; 512];

      gl::GetProgramiv(sp, gl::LINK_STATUS, &mut success);
      if success as u8 != gl::TRUE {
        gl::GetProgramInfoLog(sp, 512, null_mut(), log.as_mut_ptr());
        panic!(
          "Shader Program failed to link:\n{}",
          String::from_utf8(log.into_iter().map(|x| *x as u8).collect()).unwrap()
        );
      }

      sp
    };

    let sp2 = unsafe {
      let sp = gl::CreateProgram();
      gl::AttachShader(sp, vs);
      gl::AttachShader(sp, fs2);
      gl::LinkProgram(sp);

      let mut success = 0;
      let mut log = [0; 512];

      gl::GetProgramiv(sp, gl::LINK_STATUS, &mut success);
      if success as u8 != gl::TRUE {
        gl::GetProgramInfoLog(sp, 512, null_mut(), log.as_mut_ptr());
        panic!(
          "Shader Program failed to link:\n{}",
          String::from_utf8(log.into_iter().map(|x| *x as u8).collect()).unwrap()
        );
      }

      sp
    };

    unsafe {
      gl::DeleteShader(vs);
      gl::DeleteShader(fs1);
      gl::DeleteShader(fs2);
    }

    // Triangle 1
    let t1 = unsafe {
      let mut vertices: Vec<f32> = vec![];
      // Triangle 1
      vertices.append(&mut vec![-0.5, 0.3, 0.0]);
      vertices.append(&mut vec![0.1, 0.3, 0.0]);
      vertices.append(&mut vec![-0.2, -0.3, 0.0]);

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
        3 * mem::size_of::<f32>() as i32,
        0 as *const c_void,
      );
      gl::EnableVertexAttribArray(0);

      vao
    };

    // Triangle 2
    let t2 = unsafe {
      let mut vertices: Vec<f32> = vec![];
      // Triangle 2
      vertices.append(&mut vec![-0.1, -0.3, 0.0]);
      vertices.append(&mut vec![0.5, -0.3, 0.0]);
      vertices.append(&mut vec![0.2, 0.3, 0.0]);

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
        3 * mem::size_of::<f32>() as i32,
        0 as *const c_void,
      );
      gl::EnableVertexAttribArray(0);

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

        gl::UseProgram(sp1);

        gl::BindVertexArray(t1);
        gl::DrawArrays(gl::TRIANGLES, 0, 3);

        gl::UseProgram(sp2);

        gl::BindVertexArray(t2);
        gl::DrawArrays(gl::TRIANGLES, 0, 3);
      }

      window.swap_buffers();
    }
  } else {
    panic!("failed to create GLFW window");
  }
}
