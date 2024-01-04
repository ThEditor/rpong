use std::{fs::File, io::Read, ffi::CString};

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