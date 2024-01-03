use std::{mem, ptr};

use gl::types::{GLsizei, GLfloat};
use rpong::graphics::{
    gl_wrapper::{BufferObject, Color, VertexArrayObject, VertexAttribePointer, ShaderProgram},
    window::Window,
};

extern crate gl;
extern crate glfw;

const VERTEX_SHADER_PATH: &str = "src/shaders/vertex_shader.glsl";
const VERTEX_SHADER_PATH2: &str = "src/shaders/vertex_shader2.glsl";

const FRAGMENT_SHADER_PATH: &str = "src/shaders/fragment_shader.glsl";
const FRAGMENT_SHADER_PATH2: &str = "src/shaders/fragment_shader2.glsl";

fn main() {
    let mut window = Window::new();

    window.init_gl();

    let shader_program1 = ShaderProgram::new(VERTEX_SHADER_PATH, FRAGMENT_SHADER_PATH);
    let shader_program2 = ShaderProgram::new(VERTEX_SHADER_PATH2, FRAGMENT_SHADER_PATH2);

    let vertices1: [f32; 9] = [
        0.5, 0.5, 0.0, 0.5, -0.5, 0.0, -0.5, -0.5, 0.0
    ];

    let indices1 = [0, 1, 2];

    let vao1 = VertexArrayObject::new();
    vao1.bind();

    let vbo1 = BufferObject::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    vbo1.bind();
    vbo1.store_f32_data(&vertices1);

    let ebo1 = BufferObject::new(gl::ELEMENT_ARRAY_BUFFER, gl::STATIC_DRAW);
    ebo1.bind();
    ebo1.store_i32_data(&indices1);

    let vap1 = VertexAttribePointer::new(
        0,
        3,
        gl::FLOAT,
        gl::FALSE,
        3 * mem::size_of::<GLfloat>() as GLsizei,
        ptr::null(),
    );
    vap1.enable();

    vbo1.unbind();
    vao1.unbind();

    let vertices2: [f32; 9] = [
        0.5, 0.5, 0.0, -0.5, -0.5, 0.0, -0.5, 0.5, 0.0,
    ];

    let indices2 = [0, 1, 2];

    let vao2 = VertexArrayObject::new();
    vao2.bind();

    let vbo2 = BufferObject::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    vbo2.bind();
    vbo2.store_f32_data(&vertices2);

    let ebo2 = BufferObject::new(gl::ELEMENT_ARRAY_BUFFER, gl::STATIC_DRAW);
    ebo2.bind();
    ebo2.store_i32_data(&indices2);

    let vap2 = VertexAttribePointer::new(
        0,
        3,
        gl::FLOAT,
        gl::FALSE,
        3 * mem::size_of::<GLfloat>() as GLsizei,
        ptr::null(),
    );
    vap2.enable();

    vbo2.unbind();
    vao2.unbind();

    while !window.should_close() {
        Color::new(0.2, 0.3, 0.3, 1.0, gl::COLOR_BUFFER_BIT)
            .clear();

        shader_program1.bind();
        vao1.bind();
        unsafe { gl::DrawElements(gl::TRIANGLES, 3, gl::UNSIGNED_INT, std::ptr::null()) };
        vao1.unbind();
        shader_program1.unbind();

        shader_program2.bind();
        vao2.bind();
        unsafe { gl::DrawElements(gl::TRIANGLES, 3, gl::UNSIGNED_INT, std::ptr::null()) };
        vao2.unbind();
        shader_program2.unbind();

        window.update();
    }
}
