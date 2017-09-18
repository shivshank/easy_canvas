# Easy Canvas

Easy Canvas is a 2D Rust drawing library designed to be easy to use. Most other libraries are powerful but annoying to use for toying around and making simple examples.

Easy Canvas is inspired by the HTML5 Canvas API. Notably, it uses similar primitives (Rect, Arc, Path), but has no concept of state. Like the HTML5 Canavs, when drawing Rects, rotations can only be specified via a transform, but unlike HTML5 Canvas, the transform is not a part of the Canvas' state. Transforms must be supplied explicitly. Styles work similarly.

This Easy Canvas repo consists of two crates, `easy_canvas` and `easy_canvas_quick_window`, which is throughout the supporting documents sometimes referred to as `quick_window`.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
easy_canvas_quick_window = { git = "https://github.com/shivshank/.git" }
```

`easy_canvas_quick_window` will automatically incldue `easy_canvas` and re-export it.

Presently `easy_canvas` can only clear the screen to a given color and set a post process shader.

But **LOOK HOW EASY IT IS TO CLEAR THE SCREEN** ;p

```rust
// from the fade example

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
```

## Shader Playground

`quick_window` **also includes a shader playground binary** which you may find of interest. The interface is similar to [shadertoy.com](https://www.shadertoy.com/), but currently lacks support for many features. Your best resource for know what is and isn't supported at the moment is to check out the `create_post_process_shader` function in the `easy_canvas::drawing` module.

To build/run the binary you can run:

```
cargo build -p easy_canvas_quick_window --bin shader_playground
```

When exectued, the binary will either use the first argument as a path to a partial fragment shader file containing a `mainImage` function or look for `playground.glsl` in the current working directory.

## Building

Since the library currently depends on `glfw-rs`, you will need to be able to link with GLFW.

On Mac and Linux that means you will need to compile GLFW. On Windows, that means you can either try to compile it, let `glfw-rs` try and probably fail (you , or use a precompiled library file which is convieniently 

## Event Handling

Currently `quick_window` does not provide any way to handle events.

The front end window is however 100% separate from the canvas. If you want to roll your own event loop quickly, you can use any windowing library (such as `glutin`) and the drawing module provided behind the "drawing" feature flag in `easy_canvas`. (n.b., the drawing module is unstable and very rough around the edges, although easy to interface with; see `quick_window` for how to use it).

## Easy Canvas is a work in progress.

The API is currently unstable and I am actively looking for feedback!

`quick_window` currently uses GLFW. A GLFW precompiled lib file is provided for Windows users but Linux and Mac users will need to build GLFW themselves and override the build script.

I did add support for `glutin`, but it seemed to add significantly to start up time and memory usage, so at the moment I am not bothering with it. It also does not expose any kind of "wait for events or until timeout" sleep function like GLFW (GLFW uses WaitEvent on windows and presumably its analogue on other platforms).

Methods in `easy_canvas::Target` are most likely stable, while `easy_canvas::Canvas` methods are subject to change.

`quick_window` currently waits for events but also runs every so often to poll for drawing commands. I don't think there's any good reason that this behavior is exactly necessary. The ideal behavior is probably "tick no more often than refresh_rate seconds, sleep if there are no events or drawing commands."

But for the niche the library is targeting this seems like overkill. A simple polling event loop would suffice, but it feels rude to waste CPU cycles.

TODO:
- Make Target generic over DrawCmd
- Move DrawCmd behind a feature flag, only expose shape API by default.

- Add support for (easy) event handling.
- Clean up drawing API in `easy_canvas` (don't look at it, it's gross!).
- Move drawing module to another crate
- Path API
- C API

- Should `Host::sender` return an optional `Sender`?
