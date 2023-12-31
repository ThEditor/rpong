extern crate glfw;
extern crate gl;

use glfw::{Action, Context, Key};

const VERTEX_SHADER_SOURCE: &str = r#"   
#version 330 core
layout (location = 0) in vec3 aPos;

void main()
{
    gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
}
"#;

fn main() {
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

    let (mut window, events) = glfw.create_window(300, 300, "Pong!", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();
    
    gl::load_with(|s| window.get_proc_address(s) as *const _);

    // defining vertices

    let vertices: [f32; 9] = [
        -0.5, -0.5, 0.0,
        0.5, -0.5, 0.0,
        0.0,  0.5, 0.0
    ];

    let mut vbo: u32 = 0;
    unsafe { 
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER, (vertices.len() * std::mem::size_of::<f32>()) as isize, vertices.as_ptr() as *const std::ffi::c_void, gl::STATIC_DRAW);
    };

    // compiling vertex shaders

    let vertex_shader_source = std::ffi::CString::new(VERTEX_SHADER_SOURCE).expect("CString::new failed");
    let vertex_shader: u32;

    unsafe {
        vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        gl::ShaderSource(vertex_shader, 1, &vertex_shader_source.as_ptr(), std::ptr::null());
        gl::CompileShader(vertex_shader);

        let mut success: i32 = 0;
        gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            println!("ERROR::SHADER::VERTEX::COMPILATION_FAILED");

            let mut info_log: Vec<u8> = Vec::with_capacity(512);
            info_log.set_len(511);
            gl::GetShaderInfoLog(vertex_shader, 512, std::ptr::null_mut(), info_log.as_mut_ptr() as *mut i8);

            let info_log_str = String::from_utf8_lossy(&info_log);
            println!("{}", info_log_str);
        }
    }

    while !window.should_close() {
        window.swap_buffers();

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        }
        _ => {}
    }
}