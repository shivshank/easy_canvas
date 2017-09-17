extern crate glfw;
extern crate easy_canvas;
extern crate gl;

use easy_canvas::drawing;
use easy_canvas::{DrawCmd, Host};

use glfw::ffi::*;

use std::ffi::CString;
use std::ptr;
use std::thread;
use std::sync::mpsc::{channel, Sender};

/// Passed to the callback, used to obtain the transmitter for notifying window of draw calls.
pub struct Window {
    /// Used by one target to issue draw calls from another thread
    tx: Option<Sender<DrawCmd>>
}

impl Window {
    fn new(tx: Sender<DrawCmd>) -> Window {
        Window {
            tx: Some(tx),
        }
    }
}

impl Host for Window {
    fn sender(&mut self) -> Option<Sender<DrawCmd>> {
        self.tx.take()
    }
}

/// Create a new window that listens for draw commands issued from a thread initiated with cb.
pub fn create(title: &str, width: i32, height: i32, refresh_rate: f64, cb: fn(Window)) {
    let window = create_raw(title, width, height);
    unsafe {
        gl::Viewport(0, 0, width, height);
    }
    let (tx, rx) = channel();
    let surrogate = Window::new(tx);
    thread::spawn(move || {
        cb(surrogate);
    });
    let mut target = drawing::create_render_target(width, height);
    while unsafe { glfwWindowShouldClose(window) } == 0 {
        // process any events that happened since the last tick (roughly refresh_rate
        // seconds ago)
        let stale = drawing::parse_commands(&mut target, &rx);
        if stale {
            drawing::use_default_target();
            drawing::draw_flat_target(&target);
            unsafe { glfwSwapBuffers(window) };
        }
        drawing::print_gl_error("after render");
        unsafe { glfwWaitEventsTimeout(refresh_rate) };
    }

    drawing::delete_render_target(target);
    unsafe {
        glfwTerminate();
    }
}

fn create_raw(title: &str, width: i32, height: i32) -> *mut GLFWwindow {
    unsafe {
        if glfwInit() == 0 {
            panic!("Failed to initialize GLFW");
        }
        let title_c_str = CString::new(title).unwrap();
        let w = glfwCreateWindow(width, height, title_c_str.as_ptr() as *const _,
            ptr::null_mut(), ptr::null_mut());
        glfwMakeContextCurrent(w);
        gl::load_with(|s| {
            let c_str = CString::new(s).unwrap();
            glfwGetProcAddress(c_str.as_ptr() as _) as _
        });
        if w.is_null() {
            glfwTerminate();
            panic!("GLFW failed to create a window");
        }
        w
    }
}
