use std::ffi::CString;
use std::fs;
use std::ptr::{null, null_mut};

pub struct ShaderProgram {
  id: u32,
}

impl ShaderProgram {
  pub unsafe fn from_str(vss: &str, fss: &str) -> Self {
    // Vertex Shader
    let vsc = CString::new(vss).unwrap();
    let vsp = &vsc.as_c_str().as_ptr();

    let vs = {
      let vs_id = gl::CreateShader(gl::VERTEX_SHADER);
      gl::ShaderSource(vs_id, 1, vsp, null());
      gl::CompileShader(vs_id);

      let mut success = 0;
      let mut log = [0; 512];

      gl::GetShaderiv(vs_id, gl::COMPILE_STATUS, &mut success);
      if success as u8 != gl::TRUE {
        gl::GetShaderInfoLog(vs_id, 512, null_mut(), log.as_mut_ptr());
        panic!(
          "Vertex Shader failed to compile:\n{}",
          String::from_utf8(log.iter().map(|x| *x as u8).collect()).unwrap()
        );
      }

      vs_id
    };

    // Fragment Shader
    let fsc = CString::new(fss).unwrap();
    let fsp = &fsc.as_c_str().as_ptr();

    let fs = {
      let fs_id = gl::CreateShader(gl::FRAGMENT_SHADER);
      gl::ShaderSource(fs_id, 1, fsp, null());
      gl::CompileShader(fs_id);

      let mut success = 0;
      let mut log = [0; 512];

      gl::GetShaderiv(fs_id, gl::COMPILE_STATUS, &mut success);
      if success as u8 != gl::TRUE {
        gl::GetShaderInfoLog(fs_id, 512, null_mut(), log.as_mut_ptr());
        panic!(
          "Fragment Shader failed to compile:\n{}",
          String::from_utf8(log.iter().map(|x| *x as u8).collect()).unwrap()
        );
      }

      fs_id
    };

    // Shader Program
    let sp = {
      let sp = gl::CreateProgram();
      gl::AttachShader(sp, vs);
      gl::AttachShader(sp, fs);
      gl::LinkProgram(sp);

      let mut success = 0;
      let mut log = [0; 512];

      gl::GetProgramiv(sp, gl::LINK_STATUS, &mut success);
      if success as u8 != gl::TRUE {
        gl::GetProgramInfoLog(sp, 512, null_mut(), log.as_mut_ptr());
        panic!(
          "Shader Program failed to link:\n{}",
          String::from_utf8(log.iter().map(|x| *x as u8).collect()).unwrap()
        );
      }

      sp
    };

    // Cleanup
    gl::DeleteShader(vs);
    gl::DeleteShader(fs);

    Self { id: sp }
  }

  pub unsafe fn from_file(vsf: &str, fsf: &str) -> Self {
    let vs_code = fs::read_to_string(vsf).unwrap();
    let fs_code = fs::read_to_string(fsf).unwrap();

    Self::from_str(&vs_code, &fs_code)
  }

  pub unsafe fn use_program(&self) {
    gl::UseProgram(self.id);
  }

  pub unsafe fn set_uniform_value(&self, name: &str, value: impl UniformValue) {
    let name = CString::new(name).unwrap();
    let np = name.as_c_str().as_ptr();
    let loc = gl::GetUniformLocation(self.id, np);
    value.gl_uniform(loc);
  }
}

pub trait UniformValue: Copy {
  unsafe fn gl_uniform(self, location: i32);
}

impl UniformValue for bool {
  unsafe fn gl_uniform(self, location: i32) {
    gl::Uniform1i(location, self as i32);
  }
}

impl UniformValue for f32 {
  unsafe fn gl_uniform(self, location: i32) {
    gl::Uniform1f(location, self);
  }
}
