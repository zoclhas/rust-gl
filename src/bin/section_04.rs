use glfw::Context;

extern crate glfw;

fn main() {
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

    let (mut window, _events) = glfw
        .create_window(800, 600, "Hello!", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();
    window.swap_buffers();

    while !window.should_close() {
        glfw.poll_events();
    }
}
