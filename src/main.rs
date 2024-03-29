use std::{mem, ptr};

use gl::types::{GLsizei, GLfloat};
use rpong::graphics::window::Window;
use rusty_gl::{vertex_array_object::VertexArrayObject, buffer_object::BufferObject, vertex_attribute_pointer::VertexAttributePointer, color::Color, shader_program::ShaderProgram};

extern crate gl;
extern crate glfw;

const VERTEX_SHADER_PATH: &str = "src/shaders/vertex_shader.glsl";

const FRAGMENT_SHADER_PATH: &str = "src/shaders/fragment_shader.glsl";

fn main() {
    let mut window = Window::new();

    window.init_gl();

    let shader_program = ShaderProgram::new(VERTEX_SHADER_PATH, FRAGMENT_SHADER_PATH);
    shader_program.bind();

    let vertices1: [f32; 12] = [
        0.5, 0.5, 0.0, 0.5, -0.5, 0.0, -0.5, -0.5, 0.0, -0.5, 0.5, 0.0
    ];

    let indices1 = [0, 1, 2, 2, 3, 0];

    let vao = VertexArrayObject::new();
    vao.bind();

    let vbo = BufferObject::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    vbo.bind();
    vbo.store_f32_data(&vertices1);

    let ebo = BufferObject::new(gl::ELEMENT_ARRAY_BUFFER, gl::STATIC_DRAW);
    ebo.bind();
    ebo.store_i32_data(&indices1);

    let vap = VertexAttributePointer::new(
        0,
        3,
        gl::FLOAT,
        gl::FALSE,
        3 * mem::size_of::<GLfloat>() as GLsizei,
        ptr::null(),
    );
    vap.enable();

    vbo.unbind();
    // vao.unbind();

    while !window.should_close() {
        Color::new(0.2, 0.3, 0.3, 1.0, gl::COLOR_BUFFER_BIT)
            .clear();

        // shader_program.bind();
        // vao.bind();
        unsafe { gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null()) };
        // vao.unbind();
        // shader_program.unbind();

        window.update();
    }
}
