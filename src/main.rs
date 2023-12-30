extern crate glfw;
extern crate gl;

use glfw::{Action, Context, Key};

fn main() {
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

    let (mut window, events) = glfw.create_window(300, 300, "Pong!", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();
    
    gl::load_with(|s| window.get_proc_address(s) as *const _);

    while !window.should_close() {
        window.swap_buffers();

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        }
        glfw::WindowEvent::Key(Key::W, _, Action::Press, _) => {
            unsafe {
                gl::ClearColor(1.0, 0.0, 0.0, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }
            println!("Red");
        }
        glfw::WindowEvent::Key(Key::A, _, Action::Press, _) => {
            unsafe {
                gl::ClearColor(0.0, 1.0, 0.0, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }
            println!("Green");
        }
        glfw::WindowEvent::Key(Key::S, _, Action::Press, _) => {
            unsafe {
                gl::ClearColor(0.0, 0.0, 1.0, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }
            println!("Blue");
        }
        glfw::WindowEvent::Key(Key::D, _, Action::Press, _) => {
            unsafe {
                gl::ClearColor(1.0, 1.0, 0.0, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }
            println!("Yellow");
        }
        _ => {}
    }
}