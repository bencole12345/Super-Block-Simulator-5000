use glfw::{Action, Context};

use crate::events::{EventSource, WindowEvent};
use crate::inputs::Key;
use crate::rendering::DisplayTarget;

/// A window that will contain the game
pub struct Window {
    width: u32,
    height: u32,
    glfw_instance: glfw::Glfw,
    glfw_window: glfw::Window,
    glfw_events: std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>,
    last_mouse: (f64, f64),
}

impl Window {
    pub fn new(width: u32, height: u32, title: &str) -> Self {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));
        #[cfg(target_os = "macos")]
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

        // Enable 4x MSAA
        glfw.window_hint(glfw::WindowHint::Samples(Some(4)));

        let (mut window, events) = glfw
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window");

        window.make_current();
        window.set_key_polling(true);
        window.set_framebuffer_size_polling(true);

        window.set_cursor_pos_polling(true);
        window.set_cursor_mode(glfw::CursorMode::Disabled);
        let supported = unsafe { glfw::ffi::glfwRawMouseMotionSupported() } != 0;
        if supported {
            window.set_raw_mouse_motion(true);
        }

        // Load OpenGL functions
        // TODO: Move this into some kind of thread-safe, lazy-loaded singleton class
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        Window {
            width,
            height,
            glfw_instance: glfw,
            glfw_window: window,
            glfw_events: events,
            last_mouse: (0.0, 0.0),
        }
    }

    pub fn alive(&self) -> bool {
        !self.glfw_window.should_close()
    }
}

impl DisplayTarget for Window {
    fn swap_buffers(&mut self) {
        self.glfw_window.swap_buffers();
    }
}

impl EventSource for Window {
    fn poll_events(&mut self) -> Vec<WindowEvent> {
        self.glfw_instance.poll_events();

        let mut last_mouse_pos = self.last_mouse;

        let events: Vec<WindowEvent> = glfw::flush_messages(&self.glfw_events)
            .filter_map(|(_, event)| match event {
                glfw::WindowEvent::Key(glfw_key, _, Action::Press, _) => {
                    let key = Key::from_glfw_key(glfw_key);
                    key.map(WindowEvent::KeyPress)
                }

                glfw::WindowEvent::Key(glfw_key, _, Action::Release, _) => {
                    let key = Key::from_glfw_key(glfw_key);
                    key.map(WindowEvent::KeyRelease)
                }

                glfw::WindowEvent::CursorPos(x, y) => {
                    let (prev_x, prev_y) = self.last_mouse;
                    let dx = (x - prev_x) / (self.width as f64);
                    let dy = -1.0 * (y - prev_y) / (self.height as f64);
                    last_mouse_pos = (x, y);
                    Some(WindowEvent::MouseMove(dx as f32, dy as f32))
                }

                _ => None,
            })
            .collect();

        self.last_mouse = last_mouse_pos;

        events
    }
}
