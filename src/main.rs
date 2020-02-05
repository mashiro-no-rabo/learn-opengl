use glfw::Context;
use glfw::{OpenGlProfileHint, WindowHint, WindowMode};

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(WindowHint::ContextVersion(4, 6));
    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));

    if let Some((mut window, _recv)) =
        glfw.create_window(800, 600, "Rust-LearnOpenGL", WindowMode::Windowed)
    {
        window.make_current();
    } else {
        panic!("failed to create GLFW window");
    }
}
