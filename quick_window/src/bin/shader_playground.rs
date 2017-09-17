extern crate easy_canvas;
extern crate easy_canvas_quick_window as quick_window;

use quick_window::Window;

use std::fs::File;
use std::io::Read;

const HELP_MSG: &'static str = r#"usage: shader_playground [OPTIONS] [fragment shader source file]
If a source file is not provided explicitly, it is assumed to be playground.glsl.
See easy_canvas docs for the exact specification of how the fragment shader source file is used.
"#;

// TODO: Use lazy static to initialize a global parsed args object, make a nice CLI

fn main() {
    quick_window::create("Shader Playground", 800, 600, 1.0 / 60.0, playground);
}

fn playground(mut window: Window) {
    // first arg is the executable
    let mut args = ::std::env::args();
    args.next().unwrap();
    let source_file = args.next()
        .or(Some("playground.glsl".to_string()))
        .unwrap();

    let ctx = easy_canvas::create(&mut window, 800, 600);

    let mut frag_source = String::new();
    match File::open(source_file) {
        Ok(mut res) => {
            res.read_to_string(&mut frag_source)
                .expect("Failed to read contents.");
        },
        Err(err) => {
            eprintln!("Failed to find fragment shader file.\n{}\n", err);
            eprintln!("{}", HELP_MSG);
            ctx.stop();
            return;
        }
    }

    ctx.use_post_process(&frag_source);
}
