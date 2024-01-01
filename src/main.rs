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

const FRAGMENT_SHADER_SOURCE: &str = r#"
#version 330 core
out vec4 FragColor;

void main()
{
    FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
} 
"#;

fn main() {
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

    let (mut window, events) = glfw.create_window(300, 300, "Pong!", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();
    
    gl::load_with(|s| window.get_proc_address(s) as *const _);

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

    // compiling fragment shaders
    
    let fragment_shader_source = std::ffi::CString::new(FRAGMENT_SHADER_SOURCE).expect("CString::new failed");
    let fragment_shader: u32;
    
    unsafe {
        fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        gl::ShaderSource(fragment_shader, 1, &fragment_shader_source.as_ptr(), std::ptr::null());
        gl::CompileShader(fragment_shader);

        let mut success: i32 = 0;
        gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            println!("ERROR::SHADER::FRAGMENT::COMPILATION_FAILED");

            let mut info_log: Vec<u8> = Vec::with_capacity(512);
            info_log.set_len(511);
            gl::GetShaderInfoLog(fragment_shader, 512, std::ptr::null_mut(), info_log.as_mut_ptr() as *mut i8);

            let info_log_str = String::from_utf8_lossy(&info_log);
            println!("{}", info_log_str);
        }
    }

    // linking shader program

    let shader_program;
    unsafe {
        shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);

        let mut success: i32 = 0;
        gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
        if success == 0 {
            println!("ERROR::SHADER::PROGRAM::LINKING_FAILED");

            let mut info_log: Vec<u8> = Vec::with_capacity(512);
            info_log.set_len(511);
            gl::GetProgramInfoLog(shader_program, 512, std::ptr::null_mut(), info_log.as_mut_ptr() as *mut i8);

            let info_log_str = String::from_utf8_lossy(&info_log);
            println!("{}", info_log_str);
        }

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);
    }

    // defining vertices

    let vertices: [f32; 18] = [
        0.1, 0.7, 0.0,
        0.1, 0.1, 0.0,
        0.7, 0.1, 0.0,
        -0.1, 0.7, 0.0,
        -0.7, 0.1, 0.0,
        -0.1, 0.1, 0.0
    ];

    let indices = [
        0, 1, 2,
        3, 4, 5
    ];

    // bind vertex array object

    let mut vao: u32 = 0;
    unsafe { 
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
    }

    let mut vbo: u32 = 0;
    unsafe { 
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER, (vertices.len() * std::mem::size_of::<f32>()) as isize, vertices.as_ptr() as *const std::ffi::c_void, gl::STATIC_DRAW);

    };

    let mut ebo: u32 = 0;
    unsafe {
        gl::GenBuffers(1, &mut ebo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (indices.len() * std::mem::size_of::<f32>()) as isize, indices.as_ptr() as *const std::ffi::c_void, gl::STATIC_DRAW);
    }

    unsafe {
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * std::mem::size_of::<f32>() as gl::types::GLint, std::ptr::null());
        gl::EnableVertexAttribArray(0);
    }

    // unbind
    unsafe {
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);


            // draw triangle
            gl::UseProgram(shader_program);
            gl::BindVertexArray(vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
            gl::BindVertexArray(0);
        }

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
        glfw::WindowEvent::Key(Key::L, _, Action::Press, _) => {
            unsafe {
                let mut polygon_mode: gl::types::GLint = 0;
                gl::GetIntegerv(gl::POLYGON_MODE, &mut polygon_mode);
                if polygon_mode == gl::FILL.try_into().unwrap() {
                    gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
                }
                if polygon_mode == gl::LINE.try_into().unwrap() {
                    gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
                }
            }
        }
        _ => {}
    }
}