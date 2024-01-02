use glfw::{WindowEvent, Context, GlfwReceiver, Key, Action};

pub struct Window {
    glfw: glfw::Glfw,
    window_handle: glfw::PWindow,
    events: GlfwReceiver<(f64, WindowEvent)>,
}

impl Window {
    pub fn new() -> Window {
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

        let (mut window, events) = glfw.create_window(300, 300, "Pong!", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        window.set_key_polling(true);
        window.make_current();

        Window {
            glfw,
            window_handle: window,
            events,
        }
    }

    pub fn init_gl(&mut self) {
        self.window_handle.make_current();
        gl::load_with(|s| self.window_handle.get_proc_address(s) as *const _);
    }

    pub fn should_close(&self) -> bool {
        self.window_handle.should_close()
    }

    pub fn update(&mut self) {
        self.process_events();
        self.glfw.poll_events();
        self.window_handle.swap_buffers();
    }

    fn process_events(&mut self) {
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    unsafe { gl::Viewport(0, 0, width, height) }
                }
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    self.window_handle.set_should_close(true)
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
    }
}