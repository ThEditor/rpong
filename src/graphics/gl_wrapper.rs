use std::{os::raw::c_void, mem};

use gl::types::{GLuint, GLenum, GLboolean, GLsizei, GLfloat, GLbitfield};

pub struct VertexArrayObject {
    id: GLuint,
}

impl VertexArrayObject {
    pub fn new() -> VertexArrayObject {
        let mut id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }

        VertexArrayObject {
            id
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}
pub struct BufferObject {
    id: GLuint,
    r#type: GLenum,
    usage: GLenum,
}

impl BufferObject {
    pub fn new(r#type: gl::types::GLenum, usage: gl::types::GLenum) -> BufferObject {
        let mut id = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }
        BufferObject  {
            id,
            r#type,
            usage
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(self.r#type, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(self.r#type, 0);
        }
    }

    pub fn store_f32_data(&self, data: &[f32]) {
        unsafe {
            gl::BufferData(self.r#type, (data.len() * mem::size_of::<gl::types::GLfloat>()) as gl::types::GLsizeiptr, &data[0] as *const f32 as *const c_void, self.usage);
        }
    }

    pub fn store_i32_data(&self, data: &[i32]) {
        unsafe {
             gl::BufferData(self.r#type, (data.len() * mem::size_of::<gl::types::GLfloat>()) as gl::types::GLsizeiptr, &data[0] as *const i32 as *const c_void, self.usage);
        }
    }
}

pub struct VertexAttribePointer {
    index: GLuint,
}

impl VertexAttribePointer {
    pub fn new(
        index: u32,
        size: i32,
        r#type: GLenum,
        normalized: GLboolean,
        stride: GLsizei,
        pointer: *const c_void,
    ) -> VertexAttribePointer {
        unsafe {
            gl::VertexAttribPointer(index, size, r#type, normalized, stride, pointer);
        }
        VertexAttribePointer { index }
    }

    pub fn enable(&self) {
        unsafe {
            gl::EnableVertexAttribArray(self.index);
        }
    }

    pub fn disable(&self) {
        unsafe {
            gl::DisableVertexAttribArray(self.index);
        }
    }
}

pub struct Color {
    red: GLfloat,
    green: GLfloat,
    blue: GLfloat,
    alpha: GLfloat,
    mask: GLbitfield,
}

impl Color {
    pub fn new(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat, mask: GLbitfield) -> Color {
        Color {
            red,
            green,
            blue,
            alpha,
            mask
        }
    }

    pub fn clear(&self) {
        unsafe {
            gl::ClearColor(self.red, self.green, self.blue, self.alpha);
            gl::Clear(self.mask);
        }
    }
}