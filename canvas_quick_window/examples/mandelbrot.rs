extern crate canvas;
extern crate canvas_quick_window as quick_window;

use canvas::{Target, Color, Style};
use Style::*;

use quick_window::Window;

fn main() {
    let mut window = quick_window::create("Fractals!", 800, 600, 1.0 / 60.0, mandelbrot);
}

fn mandelbrot(window: Window) {
    let mut ctx = canvas::create(window, 800, 600);

    let style = Style::FillStyle {
        color: (200, 200, 200).normalize(),
    };

    // ctx.draw(Rect(0, 0, 10, 10), style);
    ctx.clear((200, 150, 200));
}
