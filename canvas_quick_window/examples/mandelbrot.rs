extern crate canvas;
extern crate canvas_quick_window as quick_window;

use canvas::Target;

use quick_window::Window;

use std::fs::File;
use std::io::Read;

fn main() {
    quick_window::create("Fractals!", 640, 360, 1.0 / 60.0, mandelbrot);
}

fn mandelbrot(mut window: Window) {
    let ctx = canvas::create(&mut window, 640, 360);

    let mut frag_source = String::new();
    File::open("examples/res/mandelbrot.glsl")
        .expect("Failed to find fragment shader file")
        .read_to_string(&mut frag_source)
        .expect("Failed to read contents");

    ctx.use_post_process(&frag_source);

    ctx.clear((200, 150, 200, 0.25));
}
