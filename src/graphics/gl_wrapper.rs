use std::{os::raw::c_void, mem, fs::File, io::Read, ffi::CString};

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


pub struct ShaderProgram {
    program_handle: u32,
}

impl ShaderProgram {
    pub fn new(vertex_shader_path: &str, fragment_shader_path: &str) -> ShaderProgram {
        let mut vertex_shader_file = File::open(vertex_shader_path).unwrap_or_else(|_| panic!("Failed to open {}", vertex_shader_path));
        let mut fragment_shader_file = File::open(fragment_shader_path).unwrap_or_else(|_| panic!("Failed to open {}", fragment_shader_path));
        
        let mut vertex_shader_source = String::new();
        let mut fragment_shader_source = String::new();

        vertex_shader_file.read_to_string(&mut vertex_shader_source).expect("Failed to read vertex shader");
        fragment_shader_file.read_to_string(&mut fragment_shader_source).expect("Failed to read fragment shader");
        
        unsafe {
            let c_str_vert = CString::new(vertex_shader_source.as_bytes()).unwrap();
            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(
                vertex_shader,
                1,
                &c_str_vert.as_ptr(),
                std::ptr::null(),
            );
            gl::CompileShader(vertex_shader);
            
            let c_str_frag = CString::new(fragment_shader_source.as_bytes()).unwrap();
            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(
                fragment_shader,
                1,
                &c_str_frag.as_ptr(),
                std::ptr::null(),
            );
            gl::CompileShader(fragment_shader);

            let program_handle = gl::CreateProgram();
            gl::AttachShader(program_handle, vertex_shader);
            gl::AttachShader(program_handle, fragment_shader);
            gl::LinkProgram(program_handle);
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
         
            ShaderProgram {
                program_handle
            }
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.program_handle);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::UseProgram(0);
        }
    }
}