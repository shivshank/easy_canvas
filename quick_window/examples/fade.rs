extern crate easy_canvas;
extern crate easy_canvas_quick_window as quick_window;

use easy_canvas::Target;

use quick_window::Window;

use std::time::Duration;
use std::thread::sleep;

fn main() {
    quick_window::create("Fade", 640, 360, 1.0 / 60.0, fade);
}

fn fade(mut window: Window) {
    let ctx = easy_canvas::create(&mut window, 640, 360);

    let shades = 100u32;

    for i in 0..shades {
        let alpha = i as f32 / shades as f32;
        ctx.clear((
            lerp(200.0, 60.0, alpha) as u8,
            lerp(150.0, 100.0, alpha) as u8,
            lerp(200.0, 200.0, alpha) as u8,
        ));
        sleep(Duration::from_millis(100));
    }

    println!("Ding!");

    println!("I'll only stay open for 5000 more ms, sorry!");
    sleep(Duration::from_millis(5000));
    ctx.stop();
}

fn lerp(a: f32, b: f32, alpha: f32) -> f32 {
    a as f32 + (b - a) as f32 * alpha
}