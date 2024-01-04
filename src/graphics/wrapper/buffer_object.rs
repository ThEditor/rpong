use std::{mem, ffi::c_void};

use gl::types::{GLenum, GLuint};

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