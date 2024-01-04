use gl::types::{GLbitfield, GLfloat};

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