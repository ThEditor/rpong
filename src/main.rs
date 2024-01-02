use std::{mem, ptr};

use gl::types::{GLsizei, GLfloat};
use rpong::graphics::{
    gl_wrapper::{BufferObject, Color, VertexArrayObject, VertexAttribePointer},
    window::Window,
};

extern crate gl;
extern crate glfw;

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
    let mut window = Window::new();

    window.init_gl();

    // compiling vertex shaders

    let vertex_shader_source =
        std::ffi::CString::new(VERTEX_SHADER_SOURCE).expect("CString::new failed");
    let vertex_shader: u32;

    unsafe {
        vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        gl::ShaderSource(
            vertex_shader,
            1,
            &vertex_shader_source.as_ptr(),
            std::ptr::null(),
        );
        gl::CompileShader(vertex_shader);

        let mut success: i32 = 0;
        gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            println!("ERROR::SHADER::VERTEX::COMPILATION_FAILED");

            let mut info_log: Vec<u8> = Vec::with_capacity(512);
            info_log.set_len(511);
            gl::GetShaderInfoLog(
                vertex_shader,
                512,
                std::ptr::null_mut(),
                info_log.as_mut_ptr() as *mut i8,
            );

            let info_log_str = String::from_utf8_lossy(&info_log);
            println!("{}", info_log_str);
        }
    }

    // compiling fragment shaders

    let fragment_shader_source =
        std::ffi::CString::new(FRAGMENT_SHADER_SOURCE).expect("CString::new failed");
    let fragment_shader: u32;

    unsafe {
        fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        gl::ShaderSource(
            fragment_shader,
            1,
            &fragment_shader_source.as_ptr(),
            std::ptr::null(),
        );
        gl::CompileShader(fragment_shader);

        let mut success: i32 = 0;
        gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            println!("ERROR::SHADER::FRAGMENT::COMPILATION_FAILED");

            let mut info_log: Vec<u8> = Vec::with_capacity(512);
            info_log.set_len(511);
            gl::GetShaderInfoLog(
                fragment_shader,
                512,
                std::ptr::null_mut(),
                info_log.as_mut_ptr() as *mut i8,
            );

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
            gl::GetProgramInfoLog(
                shader_program,
                512,
                std::ptr::null_mut(),
                info_log.as_mut_ptr() as *mut i8,
            );

            let info_log_str = String::from_utf8_lossy(&info_log);
            println!("{}", info_log_str);
        }

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);
    }

    // defining vertices

    let vertices: [f32; 12] = [
        0.5, 0.5, 0.0, 0.5, -0.5, 0.0, -0.5, -0.5, 0.0, -0.5, 0.5, 0.0,
    ];

    let indices = [0, 1, 2, 2, 3, 0];

    let vao = VertexArrayObject::new();
    vao.bind();

    let vbo = BufferObject::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    vbo.bind();
    vbo.store_f32_data(&vertices);

    let ebo = BufferObject::new(gl::ELEMENT_ARRAY_BUFFER, gl::STATIC_DRAW);
    ebo.bind();
    ebo.store_i32_data(&indices);

    let vap = VertexAttribePointer::new(
        0,
        3,
        gl::FLOAT,
        gl::FALSE,
        3 * mem::size_of::<GLfloat>() as GLsizei,
        ptr::null(),
    );
    vap.enable();

    vbo.unbind();
    vao.unbind();

    while !window.should_close() {
        Color::new(0.2, 0.3, 0.3, 1.0, gl::COLOR_BUFFER_BIT)
            .clear();

        unsafe {
            // draw triangle
            gl::UseProgram(shader_program);
            vao.bind();
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
            vao.unbind();
        }

        window.update();
    }
}
