use gl::types::GLchar;
use std::{ffi::CString, os::raw::c_void};

use glfw::Context;

extern crate gl;
extern crate glfw;

fn process_events(
    window: &mut glfw::Window,
    events: &glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(w, h) => unsafe {
                gl::Viewport(0, 0, w, h);
            },
            glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
                window.set_should_close(true)
            }
            _ => {}
        }
    }
}

#[rustfmt::skip]
const VERTICES: [f32; 9] = [
    -0.5, -0.5, 0.0,
    0.5, -0.5, 0.0,
    0.0, 0.5, 0.0,
];

const VERTEX_SHADER_SOURCE: &str = r"#version 330 core
layout (location = 0) in vec3 aPos;

void main()
{
    gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
}
";
const FRAGMENT_SHADER_SOURCE: &str = r"#version 330 core
out vec4 FragColor;

void main()
{
    FragColor = vec4(1.0f, 0.8f, 0.2f, 1.0f);
}
";

fn main() {
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 2));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));

    let (mut window, events) = glfw
        .create_window(800, 600, "Hello!", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    gl::load_with(|sym| window.get_proc_address(sym) as *const _);

    let shader_program = unsafe {
        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        let vertex_shader_source = CString::new(VERTEX_SHADER_SOURCE.as_bytes()).unwrap();
        gl::ShaderSource(
            vertex_shader,
            1,
            &vertex_shader_source.as_ptr(),
            std::ptr::null(),
        );
        gl::CompileShader(vertex_shader);

        let mut success = 0;
        gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);

        if success == 0 {
            let mut info_buffer = Vec::<u8>::with_capacity(512);

            gl::GetShaderInfoLog(
                vertex_shader,
                512,
                std::ptr::null_mut(),
                info_buffer.as_mut_ptr() as *mut GLchar,
            );
            eprintln!(
                "Failed to compile vertex shader:\n{}",
                std::str::from_utf8(&info_buffer).unwrap()
            );
        }

        let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        let fragment_shader_source = CString::new(FRAGMENT_SHADER_SOURCE.as_bytes()).unwrap();
        gl::ShaderSource(
            fragment_shader,
            1,
            &fragment_shader_source.as_ptr(),
            std::ptr::null(),
        );
        gl::CompileShader(fragment_shader);

        success = 0;
        gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut info_buffer = Vec::<u8>::with_capacity(512);

            gl::GetShaderInfoLog(
                fragment_shader,
                512,
                std::ptr::null_mut(),
                info_buffer.as_mut_ptr() as *mut GLchar,
            );
            eprintln!(
                "Failed to compile fragment shader:\n{}",
                std::str::from_utf8(&info_buffer).unwrap()
            );
        }

        let shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);

        success = 0;
        gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
        if success == 0 {
            let mut info_buffer = Vec::<u8>::with_capacity(512);

            gl::GetShaderInfoLog(
                shader_program,
                512,
                std::ptr::null_mut(),
                info_buffer.as_mut_ptr() as *mut GLchar,
            );
            eprintln!(
                "Failed to link shader program:\n{}",
                std::str::from_utf8(&info_buffer).unwrap()
            );
        }

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        shader_program
    };

    let vao = unsafe {
        let (mut vbo, mut vao) = (0, 0);

        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);

        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            std::mem::size_of_val(&VERTICES) as isize,
            &VERTICES as *const f32 as *const c_void,
            gl::STATIC_DRAW,
        );

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            3 * std::mem::size_of::<f32>() as i32,
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(0);

        vao
    };

    while !window.should_close() {
        process_events(&mut window, &events);

        unsafe {
            gl::ClearColor(0.3, 0.2, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::UseProgram(shader_program);
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        window.swap_buffers();
        glfw.poll_events();
    }
}
