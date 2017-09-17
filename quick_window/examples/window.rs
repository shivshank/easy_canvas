extern crate easy_canvas_quick_window as quick_window;

pub fn main() {
    quick_window::create("Hello world", 800, 600, 1.0 / 6.0, |_| {});
}
