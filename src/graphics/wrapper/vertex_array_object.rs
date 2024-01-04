use gl::types::GLuint;

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