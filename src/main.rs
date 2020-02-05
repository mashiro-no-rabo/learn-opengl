use glfw::Context;
use glfw::{OpenGlProfileHint, WindowHint, WindowMode};

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(WindowHint::ContextVersion(4, 6));
    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));

    if let Some((mut window, events)) =
        glfw.create_window(800, 600, "Rust-LearnOpenGL", WindowMode::Windowed)
    {
        window.make_current();
        gl_loader::init_gl();
        gl::Viewport::load_with(|symbol| gl_loader::get_proc_address(symbol) as *const _);
        unsafe {
            gl::Viewport(0, 0, 800, 600);
        }
        window.set_framebuffer_size_polling(true);

        while !window.should_close() {
            window.swap_buffers();

            glfw.poll_events();
            for (_, event) in glfw::flush_messages(&events) {
                println!("{:?}", event);

                match event {
                    glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
                        gl::Viewport(0, 0, width, height);
                    },
                    _ => {}
                }
            }
        }
    } else {
        panic!("failed to create GLFW window");
    }
}
